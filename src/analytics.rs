use duckdb::{params, Connection};
use regex::Regex;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct SkillRule {
    pub name: &'static str,
    pub pattern: &'static str,
    pub category: &'static str,
}

pub struct Analytics<'a> {
    conn: &'a Connection,
}

impl<'a> Analytics<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Curated list of prominent tech, AI, and data skills with word-boundary regex patterns
    pub fn skill_rules() -> Vec<SkillRule> {
        vec![
            // Programming Languages
            SkillRule { name: "Python", pattern: r"(?i)\bpython\b", category: "Language" },
            SkillRule { name: "SQL", pattern: r"(?i)\bsql\b", category: "Language" },
            SkillRule { name: "Rust", pattern: r"(?i)\brust\b", category: "Language" },
            SkillRule { name: "R", pattern: r"\b[rR]\b(?:\s+programming|\s+scripting|\s+language)?", category: "Language" },
            SkillRule { name: "Scala", pattern: r"(?i)\bscala\b", category: "Language" },
            SkillRule { name: "Java", pattern: r"(?i)\bjava\b", category: "Language" },
            SkillRule { name: "C++", pattern: r"\bC\+\+\b", category: "Language" },
            SkillRule { name: "Go/Golang", pattern: r"(?i)\b(?:golang|go\s+language)\b", category: "Language" },
            SkillRule { name: "TypeScript", pattern: r"(?i)\btypescript\b", category: "Language" },

            // AI / ML / Deep Learning
            SkillRule { name: "PyTorch", pattern: r"(?i)\bpytorch\b", category: "AI/ML" },
            SkillRule { name: "TensorFlow", pattern: r"(?i)\btensorflow\b", category: "AI/ML" },
            SkillRule { name: "Scikit-Learn", pattern: r"(?i)\b(?:scikit[-_]?learn|sklearn)\b", category: "AI/ML" },
            SkillRule { name: "LLMs / Generative AI", pattern: r"(?i)\b(?:llm|llms|generative\s+ai|genai|gpt|foundation\s+models?)\b", category: "AI/ML" },
            SkillRule { name: "NLP", pattern: r"(?i)\b(?:nlp|natural\s+language\s+processing)\b", category: "AI/ML" },
            SkillRule { name: "Computer Vision", pattern: r"(?i)\b(?:computer\s+vision|cv|image\s+recognition)\b", category: "AI/ML" },
            SkillRule { name: "Deep Learning", pattern: r"(?i)\bdeep\s+learning\b", category: "AI/ML" },
            SkillRule { name: "MLOps", pattern: r"(?i)\bmlops\b", category: "AI/ML" },
            SkillRule { name: "Hugging Face", pattern: r"(?i)\bhugging\s*face\b", category: "AI/ML" },
            SkillRule { name: "LangChain", pattern: r"(?i)\blangchain\b", category: "AI/ML" },

            // Data Engineering & Big Data
            SkillRule { name: "Apache Spark", pattern: r"(?i)\b(?:spark|pyspark)\b", category: "Data Engineering" },
            SkillRule { name: "Apache Kafka", pattern: r"(?i)\bkafka\b", category: "Data Engineering" },
            SkillRule { name: "dbt", pattern: r"(?i)\bdbt\b", category: "Data Engineering" },
            SkillRule { name: "Apache Airflow", pattern: r"(?i)\bairflow\b", category: "Data Engineering" },
            SkillRule { name: "DuckDB", pattern: r"(?i)\bduckdb\b", category: "Data Engineering" },
            SkillRule { name: "Pandas", pattern: r"(?i)\bpandas\b", category: "Data Engineering" },
            SkillRule { name: "Polars", pattern: r"(?i)\bpolars\b", category: "Data Engineering" },

            // Cloud & Infrastructure
            SkillRule { name: "AWS", pattern: r"(?i)\b(?:aws|amazon\s+web\s+services)\b", category: "Cloud/Infra" },
            SkillRule { name: "GCP", pattern: r"(?i)\b(?:gcp|google\s+cloud)\b", category: "Cloud/Infra" },
            SkillRule { name: "Azure", pattern: r"(?i)\bazure\b", category: "Cloud/Infra" },
            SkillRule { name: "Kubernetes", pattern: r"(?i)\b(?:kubernetes|k8s)\b", category: "Cloud/Infra" },
            SkillRule { name: "Docker", pattern: r"(?i)\bdocker\b", category: "Cloud/Infra" },
            SkillRule { name: "Terraform", pattern: r"(?i)\bterraform\b", category: "Cloud/Infra" },

            // Data Warehouses & Lakehouses
            SkillRule { name: "Snowflake", pattern: r"(?i)\bsnowflake\b", category: "Data Warehouse" },
            SkillRule { name: "Databricks", pattern: r"(?i)\bdatabricks\b", category: "Data Warehouse" },
            SkillRule { name: "BigQuery", pattern: r"(?i)\bbigquery\b", category: "Data Warehouse" },
            SkillRule { name: "Redshift", pattern: r"(?i)\bredshift\b", category: "Data Warehouse" },
            SkillRule { name: "PostgreSQL", pattern: r"(?i)\b(?:postgres|postgresql)\b", category: "Database" },
        ]
    }

    /// Extracts skills from stored jobs and links them into the Knowledge Graph
    pub fn extract_and_link_skills(&self) -> Result<usize, Box<dyn Error>> {
        let rules = Self::skill_rules();
        let compiled_rules: Vec<(&SkillRule, Regex)> = rules
            .iter()
            .map(|r| (r, Regex::new(r.pattern).expect("Invalid regex pattern")))
            .collect();

        // 1. Ensure Skill nodes exist in kg_nodes
        for rule in &rules {
            let skill_id = format!("skill:{}", rule.name.to_lowercase().replace([' ', '/', '-'], "_"));
            self.conn.execute(
                r#"
                INSERT OR REPLACE INTO kg_nodes (node_id, node_type, name, properties)
                VALUES (?, 'Skill', ?, CONCAT('{"category":"', ?, '"}'));
                "#,
                params![skill_id, rule.name, rule.category],
            )?;
        }

        // 2. Scan all jobs in DuckDB
        let mut stmt = self.conn.prepare(
            "SELECT id, keyword, company_name, title, raw_json FROM jobs;"
        )?;

        let mut rows = stmt.query([])?;
        let mut extracted_count = 0;

        while let Some(row) = rows.next()? {
            let job_id: i64 = row.get(0)?;
            let keyword: String = row.get(1)?;
            let company_name: String = row.get(2)?;
            let title: String = row.get(3)?;
            let raw_json: String = row.get(4)?;

            let search_text = format!("{} {}", title, raw_json);

            for (rule, re) in &compiled_rules {
                if re.is_match(&search_text) {
                    let skill_id = format!("skill:{}", rule.name.to_lowercase().replace([' ', '/', '-'], "_"));

                    // Edge: Job -> REQUIRES_SKILL -> Skill
                    self.conn.execute(
                        r#"
                        INSERT OR REPLACE INTO kg_edges (source_id, target_id, relation, weight, properties)
                        VALUES (?, ?, 'REQUIRES_SKILL', 1, '{}');
                        "#,
                        params![format!("job:{}", job_id), skill_id],
                    )?;

                    // Edge: Keyword -> DEMANDS_SKILL -> Skill (aggregated later)
                    if !keyword.is_empty() {
                        let kw_id = format!("keyword:{}", keyword.trim().to_lowercase());
                        self.conn.execute(
                            r#"
                            INSERT INTO kg_edges (source_id, target_id, relation, weight, properties)
                            VALUES (?, ?, 'DEMANDS_SKILL', 1, '{}')
                            ON CONFLICT (source_id, target_id, relation) DO UPDATE SET
                                weight = kg_edges.weight + 1;
                            "#,
                            params![kw_id, skill_id],
                        )?;
                    }

                    // Edge: Company -> SEEKS_SKILL -> Skill (aggregated later)
                    if !company_name.is_empty() {
                        let comp_id = format!("company:{}", company_name.trim().to_lowercase());
                        self.conn.execute(
                            r#"
                            INSERT INTO kg_edges (source_id, target_id, relation, weight, properties)
                            VALUES (?, ?, 'SEEKS_SKILL', 1, '{}')
                            ON CONFLICT (source_id, target_id, relation) DO UPDATE SET
                                weight = kg_edges.weight + 1;
                            "#,
                            params![comp_id, skill_id],
                        )?;
                    }

                    extracted_count += 1;
                }
            }
        }

        Ok(extracted_count)
    }

    /// Top skills by role/keyword with percentage of jobs demanding it
    pub fn top_skills_by_role(&self, keyword: Option<&str>, limit: usize) -> Result<Vec<(String, usize, f64)>, Box<dyn Error>> {
        let (sql, kw_param) = match keyword {
            Some(kw) => (
                r#"
                WITH role_total AS (
                    SELECT COUNT(*) as total FROM jobs WHERE lower(trim(keyword)) = lower(trim(?))
                )
                SELECT s.name, COUNT(DISTINCT e.source_id) as job_count,
                       ROUND(COUNT(DISTINCT e.source_id) * 100.0 / NULLIF(MAX(r.total), 0), 1) as pct
                FROM kg_edges e
                JOIN kg_nodes s ON e.target_id = s.node_id AND s.node_type = 'Skill'
                JOIN jobs j ON e.source_id = 'job:' || CAST(j.id AS VARCHAR)
                CROSS JOIN role_total r
                WHERE e.relation = 'REQUIRES_SKILL' AND lower(trim(j.keyword)) = lower(trim(?))
                GROUP BY s.name
                ORDER BY job_count DESC, s.name ASC
                LIMIT ?;
                "#,
                Some(kw.to_string()),
            ),
            None => (
                r#"
                WITH total_jobs AS (SELECT COUNT(*) as total FROM jobs)
                SELECT s.name, COUNT(DISTINCT e.source_id) as job_count,
                       ROUND(COUNT(DISTINCT e.source_id) * 100.0 / NULLIF(MAX(t.total), 0), 1) as pct
                FROM kg_edges e
                JOIN kg_nodes s ON e.target_id = s.node_id AND s.node_type = 'Skill'
                CROSS JOIN total_jobs t
                WHERE e.relation = 'REQUIRES_SKILL'
                GROUP BY s.name
                ORDER BY job_count DESC, s.name ASC
                LIMIT ?;
                "#,
                None,
            ),
        };

        let mut res = Vec::new();
        if let Some(kw) = kw_param {
            let mut stmt = self.conn.prepare(sql)?;
            let rows = stmt.query_map(params![kw, kw, limit as i64], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })?;
            for r in rows {
                res.push(r?);
            }
        } else {
            let mut stmt = self.conn.prepare(sql)?;
            let rows = stmt.query_map(params![limit as i64], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })?;
            for r in rows {
                res.push(r?);
            }
        }

        Ok(res)
    }

    /// Skill Co-occurrence Network: Pairs of skills most frequently co-demanded in the same job
    pub fn skill_co_occurrences(&self, limit: usize) -> Result<Vec<(String, String, usize)>, Box<dyn Error>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT s1.name as skill_a, s2.name as skill_b, COUNT(*) as pair_count
            FROM kg_edges e1
            JOIN kg_edges e2 ON e1.source_id = e2.source_id AND e1.target_id < e2.target_id
            JOIN kg_nodes s1 ON e1.target_id = s1.node_id AND s1.node_type = 'Skill'
            JOIN kg_nodes s2 ON e2.target_id = s2.node_id AND s2.node_type = 'Skill'
            WHERE e1.relation = 'REQUIRES_SKILL' AND e2.relation = 'REQUIRES_SKILL'
            GROUP BY s1.name, s2.name
            ORDER BY pair_count DESC, s1.name ASC
            LIMIT ?;
            "#,
        )?;

        let rows = stmt.query_map(params![limit as i64], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;

        let mut res = Vec::new();
        for r in rows {
            res.push(r?);
        }
        Ok(res)
    }

    // ==========================================
    // Time-Series Analytics
    // ==========================================

    /// Daily job posting trend per keyword/role
    pub fn daily_posting_trends(&self) -> Result<Vec<(String, String, usize)>, Box<dyn Error>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT 
                SUBSTRING(posted_at, 1, 10) as post_date,
                keyword,
                COUNT(*) as job_count
            FROM jobs
            WHERE posted_at IS NOT NULL AND length(posted_at) >= 10
            GROUP BY 1, 2
            ORDER BY post_date DESC, job_count DESC;
            "#,
        )?;

        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;

        let mut res = Vec::new();
        for r in rows {
            res.push(r?);
        }
        Ok(res)
    }

    /// Day of the week posting distribution (percentage of total jobs posted on Monday, Tuesday, etc.)
    pub fn day_of_week_distribution(&self) -> Result<Vec<(String, usize, f64)>, Box<dyn Error>> {
        let mut stmt = self.conn.prepare(
            r#"
            WITH day_counts AS (
                SELECT 
                    dayname(TRY_CAST(SUBSTRING(posted_at, 1, 10) AS DATE)) as day_name,
                    COUNT(*) as cnt
                FROM jobs
                WHERE posted_at IS NOT NULL AND length(posted_at) >= 10
                GROUP BY 1
            ),
            totals AS (
                SELECT SUM(cnt) as total_sum FROM day_counts
            )
            SELECT d.day_name, d.cnt, ROUND(d.cnt * 100.0 / t.total_sum, 1) as pct
            FROM day_counts d
            CROSS JOIN totals t
            WHERE d.day_name IS NOT NULL
            ORDER BY d.cnt DESC;
            "#,
        )?;

        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;

        let mut res = Vec::new();
        for r in rows {
            res.push(r?);
        }
        Ok(res)
    }

    /// Daily intake velocity and cumulative job volume
    pub fn daily_velocity(&self) -> Result<Vec<(String, usize, usize)>, Box<dyn Error>> {
        let mut stmt = self.conn.prepare(
            r#"
            WITH daily AS (
                SELECT 
                    SUBSTRING(posted_at, 1, 10) as post_date,
                    COUNT(*) as daily_count
                FROM jobs
                WHERE posted_at IS NOT NULL AND length(posted_at) >= 10
                GROUP BY 1
            )
            SELECT 
                post_date,
                daily_count,
                SUM(daily_count) OVER (ORDER BY post_date ASC) as cumulative_count
            FROM daily
            ORDER BY post_date DESC;
            "#,
        )?;

        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;

        let mut res = Vec::new();
        for r in rows {
            res.push(r?);
        }
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::JobDb;

    #[test]
    fn test_analytics_and_skills() -> Result<(), Box<dyn Error>> {
        let db = JobDb::open_in_memory()?;
        let sample_job1 = serde_json::json!({
            "id": 101,
            "title": "Senior AI Engineer (Python, PyTorch, LLMs)",
            "url": "https://example.com/101",
            "created_at": "2026-11-09T14:00:00Z",
            "salary": "£120,000",
            "location": "London",
            "company": {"trading_name": "OpenAI"},
            "seniority": {"display_name": "Senior"},
            "category": {"display_name": "AI"},
            "type": {"display_name": "Full-time"}
        });

        let sample_job2 = serde_json::json!({
            "id": 102,
            "title": "Lead Data Engineer with Python, Spark, and dbt",
            "url": "https://example.com/102",
            "created_at": "2026-11-08T11:00:00Z",
            "salary": "£105,000",
            "location": "London",
            "company": {"trading_name": "Databricks"},
            "seniority": {"display_name": "Lead"},
            "category": {"display_name": "Data"},
            "type": {"display_name": "Full-time"}
        });

        db.upsert_job("ai engineer", &sample_job1)?;
        db.upsert_job("data engineer", &sample_job2)?;

        let kg = db.knowledge_graph();
        kg.project_from_jobs()?;

        let analytics = db.analytics();
        let extracted = analytics.extract_and_link_skills()?;
        assert!(extracted >= 5, "Should extract at least Python, PyTorch, LLMs, Spark, dbt");

        let top_ai_skills = analytics.top_skills_by_role(Some("ai engineer"), 5)?;
        assert!(!top_ai_skills.is_empty());
        assert!(top_ai_skills.iter().any(|(s, _, _)| s == "Python"));

        let co_occur = analytics.skill_co_occurrences(5)?;
        assert!(!co_occur.is_empty());

        let days = analytics.day_of_week_distribution()?;
        assert!(!days.is_empty());

        let velocity = analytics.daily_velocity()?;
        assert!(!velocity.is_empty());

        Ok(())
    }
}
