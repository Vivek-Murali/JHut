use duckdb::{params, Connection};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KgNodeView {
    pub id: String,
    pub label: String,
    pub group: String, // "company", "role", "skill"
    pub value: usize,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KgEdgeView {
    pub from: String,
    pub to: String,
    pub label: String,
    pub value: usize,
}

#[derive(Debug, Default)]
pub struct GraphStats {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub node_counts_by_type: Vec<(String, usize)>,
    pub edge_counts_by_relation: Vec<(String, usize)>,
}

#[derive(Debug)]
pub struct ConnectedEdge {
    pub source: String,
    pub relation: String,
    pub target: String,
    pub weight: i64,
}

pub struct KnowledgeGraph<'a> {
    conn: &'a Connection,
}

impl<'a> KnowledgeGraph<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Initializes graph schema (kg_nodes and kg_edges)
    pub fn init_schema(&self) -> Result<(), Box<dyn Error>> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS kg_nodes (
                node_id VARCHAR PRIMARY KEY,
                node_type VARCHAR NOT NULL,
                name VARCHAR NOT NULL,
                properties VARCHAR
            );

            CREATE TABLE IF NOT EXISTS kg_edges (
                source_id VARCHAR NOT NULL,
                target_id VARCHAR NOT NULL,
                relation VARCHAR NOT NULL,
                weight BIGINT DEFAULT 1,
                properties VARCHAR,
                PRIMARY KEY (source_id, target_id, relation)
            );

            CREATE INDEX IF NOT EXISTS idx_kg_nodes_type ON kg_nodes(node_type);
            CREATE INDEX IF NOT EXISTS idx_kg_edges_source ON kg_edges(source_id);
            CREATE INDEX IF NOT EXISTS idx_kg_edges_target ON kg_edges(target_id);
            CREATE INDEX IF NOT EXISTS idx_kg_edges_relation ON kg_edges(relation);
            "#,
        )?;
        Ok(())
    }

    /// Projects all jobs from DuckDB into the Knowledge Graph (Nodes & Edges)
    pub fn project_from_jobs(&self) -> Result<GraphStats, Box<dyn Error>> {
        self.init_schema()?;

        // Clean out existing projected graph so we do a fresh sync
        self.conn.execute_batch(
            r#"
            DELETE FROM kg_edges;
            DELETE FROM kg_nodes;
            "#,
        )?;

        // 1. Insert Nodes
        // Job nodes
        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO kg_nodes (node_id, node_type, name, properties)
            SELECT
                'job:' || CAST(id AS VARCHAR) AS node_id,
                'Job' AS node_type,
                title AS name,
                CONCAT('{"url":"', replace(url, '"', '\"'), '","salary":"', replace(salary, '"', '\"'), '","location":"', replace(location, '"', '\"'), '"}') AS properties
            FROM jobs;
            "#,
            [],
        )?;

        // Company nodes
        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO kg_nodes (node_id, node_type, name, properties)
            SELECT
                'company:' || lower(trim(company_name)) AS node_id,
                'Company' AS node_type,
                company_name AS name,
                CONCAT('{"url":"', replace(MAX(company_url), '"', '\"'), '","licensed_sponsor":', CASE WHEN bool_or(licensed_sponsor) THEN 'true' ELSE 'false' END, '}') AS properties
            FROM jobs
            WHERE company_name IS NOT NULL AND length(trim(company_name)) > 0
            GROUP BY lower(trim(company_name)), company_name;
            "#,
            [],
        )?;

        // Location nodes
        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO kg_nodes (node_id, node_type, name, properties)
            SELECT
                'location:' || lower(trim(location)) AS node_id,
                'Location' AS node_type,
                location AS name,
                '{}' AS properties
            FROM jobs
            WHERE location IS NOT NULL AND length(trim(location)) > 0
            GROUP BY lower(trim(location)), location;
            "#,
            [],
        )?;

        // Keyword nodes
        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO kg_nodes (node_id, node_type, name, properties)
            SELECT
                'keyword:' || lower(trim(keyword)) AS node_id,
                'Keyword' AS node_type,
                keyword AS name,
                '{}' AS properties
            FROM jobs
            WHERE keyword IS NOT NULL AND length(trim(keyword)) > 0
            GROUP BY lower(trim(keyword)), keyword;
            "#,
            [],
        )?;

        // Seniority nodes
        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO kg_nodes (node_id, node_type, name, properties)
            SELECT
                'seniority:' || lower(trim(seniority)) AS node_id,
                'Seniority' AS node_type,
                seniority AS name,
                '{}' AS properties
            FROM jobs
            WHERE seniority IS NOT NULL AND length(trim(seniority)) > 0
            GROUP BY lower(trim(seniority)), seniority;
            "#,
            [],
        )?;

        // Category nodes
        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO kg_nodes (node_id, node_type, name, properties)
            SELECT
                'category:' || lower(trim(category)) AS node_id,
                'Category' AS node_type,
                category AS name,
                '{}' AS properties
            FROM jobs
            WHERE category IS NOT NULL AND length(trim(category)) > 0
            GROUP BY lower(trim(category)), category;
            "#,
            [],
        )?;

        // Visa sponsorship target node
        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO kg_nodes (node_id, node_type, name, properties)
            VALUES ('visa:uk_sponsorship', 'VisaSponsorship', 'UK Skilled Worker Visa Sponsorship', '{}');
            "#,
            [],
        )?;

        // 2. Insert Edges
        // Company -> POSTED -> Job
        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO kg_edges (source_id, target_id, relation, weight, properties)
            SELECT
                'company:' || lower(trim(company_name)) AS source_id,
                'job:' || CAST(id AS VARCHAR) AS target_id,
                'POSTED' AS relation,
                1 AS weight,
                '{}' AS properties
            FROM jobs
            WHERE company_name IS NOT NULL AND length(trim(company_name)) > 0;
            "#,
            [],
        )?;

        // Job -> MATCHES_KEYWORD -> Keyword
        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO kg_edges (source_id, target_id, relation, weight, properties)
            SELECT
                'job:' || CAST(id AS VARCHAR) AS source_id,
                'keyword:' || lower(trim(keyword)) AS target_id,
                'MATCHES_KEYWORD' AS relation,
                1 AS weight,
                '{}' AS properties
            FROM jobs
            WHERE keyword IS NOT NULL AND length(trim(keyword)) > 0;
            "#,
            [],
        )?;

        // Job -> LOCATED_IN -> Location
        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO kg_edges (source_id, target_id, relation, weight, properties)
            SELECT
                'job:' || CAST(id AS VARCHAR) AS source_id,
                'location:' || lower(trim(location)) AS target_id,
                'LOCATED_IN' AS relation,
                1 AS weight,
                '{}' AS properties
            FROM jobs
            WHERE location IS NOT NULL AND length(trim(location)) > 0;
            "#,
            [],
        )?;

        // Job -> REQUIRES_SENIORITY -> Seniority
        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO kg_edges (source_id, target_id, relation, weight, properties)
            SELECT
                'job:' || CAST(id AS VARCHAR) AS source_id,
                'seniority:' || lower(trim(seniority)) AS target_id,
                'REQUIRES_SENIORITY' AS relation,
                1 AS weight,
                '{}' AS properties
            FROM jobs
            WHERE seniority IS NOT NULL AND length(trim(seniority)) > 0;
            "#,
            [],
        )?;

        // Job -> IN_CATEGORY -> Category
        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO kg_edges (source_id, target_id, relation, weight, properties)
            SELECT
                'job:' || CAST(id AS VARCHAR) AS source_id,
                'category:' || lower(trim(category)) AS target_id,
                'IN_CATEGORY' AS relation,
                1 AS weight,
                '{}' AS properties
            FROM jobs
            WHERE category IS NOT NULL AND length(trim(category)) > 0;
            "#,
            [],
        )?;

        // Company -> HIRES_ROLE -> Keyword (direct multi-edge aggregate)
        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO kg_edges (source_id, target_id, relation, weight, properties)
            SELECT
                'company:' || lower(trim(company_name)) AS source_id,
                'keyword:' || lower(trim(keyword)) AS target_id,
                'HIRES_ROLE' AS relation,
                COUNT(*) AS weight,
                CONCAT('{"open_roles":', CAST(COUNT(*) AS VARCHAR), '}') AS properties
            FROM jobs
            WHERE company_name IS NOT NULL AND length(trim(company_name)) > 0
              AND keyword IS NOT NULL AND length(trim(keyword)) > 0
            GROUP BY lower(trim(company_name)), lower(trim(keyword));
            "#,
            [],
        )?;

        // Company -> OFFERS_SPONSORSHIP -> VisaSponsorship
        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO kg_edges (source_id, target_id, relation, weight, properties)
            SELECT
                'company:' || lower(trim(company_name)) AS source_id,
                'visa:uk_sponsorship' AS target_id,
                'OFFERS_SPONSORSHIP' AS relation,
                COUNT(*) AS weight,
                CONCAT('{"sponsored_jobs":', CAST(COUNT(*) AS VARCHAR), '}') AS properties
            FROM jobs
            WHERE company_name IS NOT NULL AND length(trim(company_name)) > 0
              AND licensed_sponsor = true
            GROUP BY lower(trim(company_name));
            "#,
            [],
        )?;

        self.get_stats()
    }

    /// Fetches overall Knowledge Graph stats
    pub fn get_stats(&self) -> Result<GraphStats, Box<dyn Error>> {
        let total_nodes: usize = self
            .conn
            .query_row("SELECT COUNT(*) FROM kg_nodes;", [], |row| row.get(0))?;

        let total_edges: usize = self
            .conn
            .query_row("SELECT COUNT(*) FROM kg_edges;", [], |row| row.get(0))?;

        let mut stmt = self
            .conn
            .prepare("SELECT node_type, COUNT(*) FROM kg_nodes GROUP BY node_type ORDER BY COUNT(*) DESC;")?;
        let node_rows = stmt.query_map([], |row| {
            let t: String = row.get(0)?;
            let c: usize = row.get(1)?;
            Ok((t, c))
        })?;
        let mut node_counts = Vec::new();
        for r in node_rows {
            node_counts.push(r?);
        }

        let mut stmt2 = self
            .conn
            .prepare("SELECT relation, COUNT(*) FROM kg_edges GROUP BY relation ORDER BY COUNT(*) DESC;")?;
        let edge_rows = stmt2.query_map([], |row| {
            let rel: String = row.get(0)?;
            let c: usize = row.get(1)?;
            Ok((rel, c))
        })?;
        let mut edge_counts = Vec::new();
        for r in edge_rows {
            edge_counts.push(r?);
        }

        Ok(GraphStats {
            total_nodes,
            total_edges,
            node_counts_by_type: node_counts,
            edge_counts_by_relation: edge_counts,
        })
    }

    /// Knowledge Graph Query: Top hiring companies for a given role/keyword
    pub fn top_hiring_companies(&self, keyword: Option<&str>, limit: usize) -> Result<Vec<(String, i64)>, Box<dyn Error>> {
        let (sql, kw_param) = match keyword {
            Some(kw) => (
                r#"
                SELECT n.name, e.weight
                FROM kg_edges e
                JOIN kg_nodes n ON e.source_id = n.node_id
                WHERE e.relation = 'HIRES_ROLE' AND e.target_id = 'keyword:' || lower(trim(?))
                ORDER BY e.weight DESC, n.name ASC
                LIMIT ?;
                "#,
                Some(kw.to_string()),
            ),
            None => (
                r#"
                SELECT n.name, SUM(e.weight) as total_roles
                FROM kg_edges e
                JOIN kg_nodes n ON e.source_id = n.node_id
                WHERE e.relation = 'HIRES_ROLE'
                GROUP BY n.name
                ORDER BY total_roles DESC, n.name ASC
                LIMIT ?;
                "#,
                None,
            ),
        };

        let mut res = Vec::new();
        if let Some(kw) = kw_param {
            let mut stmt = self.conn.prepare(sql)?;
            let rows = stmt.query_map(params![kw, limit as i64], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })?;
            for r in rows {
                res.push(r?);
            }
        } else {
            let mut stmt = self.conn.prepare(sql)?;
            let rows = stmt.query_map(params![limit as i64], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })?;
            for r in rows {
                res.push(r?);
            }
        }

        Ok(res)
    }

    /// Knowledge Graph Query: Top visa-sponsoring companies across London or other locations
    pub fn top_sponsoring_companies_in_location(&self, location_substr: &str, limit: usize) -> Result<Vec<(String, i64)>, Box<dyn Error>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT c.name, COUNT(DISTINCT j.node_id) as roles_count
            FROM kg_nodes c
            JOIN kg_edges e_post ON e_post.source_id = c.node_id AND e_post.relation = 'POSTED'
            JOIN kg_nodes j ON e_post.target_id = j.node_id
            JOIN kg_edges e_loc ON e_loc.source_id = j.node_id AND e_loc.relation = 'LOCATED_IN'
            JOIN kg_edges e_spons ON e_spons.source_id = c.node_id AND e_spons.relation = 'OFFERS_SPONSORSHIP'
            WHERE lower(e_loc.target_id) LIKE '%' || lower(?) || '%'
            GROUP BY c.name
            ORDER BY roles_count DESC, c.name ASC
            LIMIT ?;
            "#,
        )?;

        let rows = stmt.query_map(params![location_substr, limit as i64], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;

        let mut res = Vec::new();
        for r in rows {
            res.push(r?);
        }
        Ok(res)
    }

    /// Inspect 1-hop subgraph relationships for any given node
    pub fn get_subgraph_edges(&self, node_id: &str) -> Result<Vec<ConnectedEdge>, Box<dyn Error>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT source_id, relation, target_id, weight
            FROM kg_edges
            WHERE source_id = ? OR target_id = ?
            ORDER BY relation, target_id;
            "#,
        )?;

        let rows = stmt.query_map(params![node_id, node_id], |row| {
            Ok(ConnectedEdge {
                source: row.get(0)?,
                relation: row.get(1)?,
                target: row.get(2)?,
                weight: row.get(3)?,
            })
        })?;

        let mut res = Vec::new();
        for r in rows {
            res.push(r?);
        }
        Ok(res)
    }

    /// Extracts nodes and edges specifically for the visual Lineage Knowledge Graph (Companies -> Roles -> Skills)
    pub fn get_lineage_graph_data(
        &self,
        max_companies: usize,
        max_skills: usize,
    ) -> Result<(Vec<KgNodeView>, Vec<KgEdgeView>), Box<dyn Error>> {
        let mut nodes = Vec::new();
        let mut node_ids = std::collections::HashSet::new();

        // 1. Role Nodes (Fixed Core Hubs)
        let roles = vec![
            ("keyword:ai engineer", "AI Engineer", 32),
            ("keyword:data scientist", "Data Scientist", 28),
            ("keyword:data engineer", "Data Engineer", 32),
        ];
        for (id, label, val) in roles {
            nodes.push(KgNodeView {
                id: id.to_string(),
                label: label.to_string(),
                group: "role".to_string(),
                value: val,
                title: format!("Target Search Role: {}", label),
            });
            node_ids.insert(id.to_string());
        }

        // 2. Top Companies
        let mut comp_stmt = self.conn.prepare(
            r#"
            SELECT e.source_id, n.name, SUM(e.weight) as total_roles
            FROM kg_edges e
            JOIN kg_nodes n ON e.source_id = n.node_id
            WHERE e.relation = 'HIRES_ROLE'
            GROUP BY e.source_id, n.name
            ORDER BY total_roles DESC
            LIMIT ?;
            "#,
        )?;
        let comp_rows = comp_stmt.query_map(params![max_companies as i64], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let count: i64 = row.get(2)?;
            Ok((id, name, count as usize))
        })?;

        for r in comp_rows {
            let (id, name, count) = r?;
            nodes.push(KgNodeView {
                id: id.clone(),
                label: name.clone(),
                group: "company".to_string(),
                value: 16 + count.min(24),
                title: format!("Employer: {} ({} active openings)", name, count),
            });
            node_ids.insert(id);
        }

        // 3. Top Skills
        let mut skill_stmt = self.conn.prepare(
            r#"
            SELECT e.target_id, s.name, COUNT(*) as demand_count
            FROM kg_edges e
            JOIN kg_nodes s ON e.target_id = s.node_id
            WHERE e.relation = 'REQUIRES_SKILL'
            GROUP BY e.target_id, s.name
            ORDER BY demand_count DESC
            LIMIT ?;
            "#,
        )?;
        let skill_rows = skill_stmt.query_map(params![max_skills as i64], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let count: i64 = row.get(2)?;
            Ok((id, name, count as usize))
        })?;

        for r in skill_rows {
            let (id, name, count) = r?;
            nodes.push(KgNodeView {
                id: id.clone(),
                label: name.clone(),
                group: "skill".to_string(),
                value: 12 + count.min(20),
                title: format!("Skill: {} (Demanded in {} jobs)", name, count),
            });
            node_ids.insert(id);
        }

        // 4. Edges linking Companies -> Roles, Roles -> Skills, and Companies -> Skills
        let mut edges = Vec::new();
        let mut edge_keys = std::collections::HashSet::new();

        let mut edge_stmt = self.conn.prepare(
            r#"
            SELECT source_id, target_id, relation, weight
            FROM kg_edges
            WHERE relation IN ('HIRES_ROLE', 'DEMANDS_SKILL', 'SEEKS_SKILL')
            ORDER BY weight DESC;
            "#,
        )?;

        let edge_rows = edge_stmt.query_map([], |row| {
            let from: String = row.get(0)?;
            let to: String = row.get(1)?;
            let relation: String = row.get(2)?;
            let weight: i64 = row.get(3)?;
            Ok((from, to, relation, weight as usize))
        })?;

        for r in edge_rows {
            let (from, to, relation, weight) = r?;
            if node_ids.contains(&from) && node_ids.contains(&to) {
                let key = format!("{}-{}-{}", from, to, relation);
                if !edge_keys.contains(&key) {
                    edge_keys.insert(key);
                    edges.push(KgEdgeView {
                        from,
                        to,
                        label: relation,
                        value: weight.clamp(1, 6),
                    });
                }
            }
        }

        Ok((nodes, edges))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::JobDb;

    #[test]
    fn test_knowledge_graph_projection() -> Result<(), Box<dyn Error>> {
        let db = JobDb::open_in_memory()?;
        let sample_job = serde_json::json!({
            "id": 999,
            "title": "Lead AI Engineer",
            "url": "https://example.com/job/999",
            "created_at": "2026-11-09T10:00:00Z",
            "salary": "£120,000",
            "location": "London",
            "company": {
                "trading_name": "DeepAI Corp",
                "licensed_sponsor": true,
                "url": "https://deepai.corp"
            },
            "seniority": {"display_name": "Lead"},
            "category": {"display_name": "Engineering"},
            "type": {"display_name": "Full-time"}
        });

        db.upsert_job("ai engineer", &sample_job)?;

        let kg = db.knowledge_graph();
        let stats = kg.project_from_jobs()?;

        assert!(stats.total_nodes > 0, "Nodes should be projected");
        assert!(stats.total_edges > 0, "Edges should be projected");

        let top_companies = kg.top_hiring_companies(Some("ai engineer"), 5)?;
        assert_eq!(top_companies.len(), 1);
        assert_eq!(top_companies[0].0, "DeepAI Corp");

        let analytics = db.analytics();
        analytics.extract_and_link_skills()?;

        let (lineage_nodes, lineage_edges) = kg.get_lineage_graph_data(10, 10)?;
        assert!(!lineage_nodes.is_empty(), "Should return lineage nodes");
        assert!(!lineage_edges.is_empty(), "Should return lineage edges");

        Ok(())
    }
}
