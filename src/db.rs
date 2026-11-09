use duckdb::{params, Connection};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct JobSummary {
    pub id: i64,
    pub keyword: String,
    pub title: String,
    pub company_name: String,
    pub location: String,
    pub salary: String,
    pub url: String,
    pub fetched_at: String,
}

pub struct JobDb {
    conn: Connection,
}

impl JobDb {
    /// Opens or creates a persistent DuckDB database at the given path.
    pub fn open(path: &str) -> Result<Self, Box<dyn Error>> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.init()?;
        Ok(db)
    }

    /// Opens an in-memory DuckDB database (useful for unit testing).
    pub fn open_in_memory() -> Result<Self, Box<dyn Error>> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.init()?;
        Ok(db)
    }

    /// Returns reference to internal DuckDB connection
    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    /// Creates a KnowledgeGraph projector for this database
    pub fn knowledge_graph(&self) -> crate::graph::KnowledgeGraph<'_> {
        crate::graph::KnowledgeGraph::new(&self.conn)
    }

    /// Creates an Analytics suite for this database
    pub fn analytics(&self) -> crate::analytics::Analytics<'_> {
        crate::analytics::Analytics::new(&self.conn)
    }

    /// Creates the table schema if it does not already exist.
    pub fn init(&self) -> Result<(), Box<dyn Error>> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS jobs (
                id BIGINT PRIMARY KEY,
                keyword VARCHAR,
                title VARCHAR,
                company_name VARCHAR,
                company_url VARCHAR,
                licensed_sponsor BOOLEAN,
                location VARCHAR,
                salary VARCHAR,
                seniority VARCHAR,
                category VARCHAR,
                employment_type VARCHAR,
                url VARCHAR,
                posted_at VARCHAR,
                fetched_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                raw_json TEXT
            );
            CREATE INDEX IF NOT EXISTS idx_jobs_keyword ON jobs(keyword);
            CREATE INDEX IF NOT EXISTS idx_jobs_fetched_at ON jobs(fetched_at);
            "#,
        )?;
        Ok(())
    }

    /// Upserts a single job record.
    pub fn upsert_job(&self, keyword: &str, job: &serde_json::Value) -> Result<(), Box<dyn Error>> {
        let id = job
            .get("id")
            .and_then(|v| v.as_i64())
            .ok_or("Job missing id")?;
        let title = job.get("title").and_then(|v| v.as_str()).unwrap_or("");
        let company_name = job
            .get("company")
            .and_then(|c| c.get("trading_name"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let company_url = job
            .get("company")
            .and_then(|c| c.get("url"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let licensed_sponsor = job
            .get("company")
            .and_then(|c| c.get("licensed_sponsor"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let location = job.get("location").and_then(|v| v.as_str()).unwrap_or("");
        let salary = job.get("salary").and_then(|v| v.as_str()).unwrap_or("");
        let seniority = job
            .get("seniority")
            .and_then(|s| s.get("display_name"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let category = job
            .get("category")
            .and_then(|c| c.get("display_name"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let employment_type = job
            .get("type")
            .and_then(|t| t.get("display_name"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let url = job.get("url").and_then(|v| v.as_str()).unwrap_or("");
        let posted_at = job
            .get("created_at")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let raw_json = serde_json::to_string(job)?;

        self.conn.execute(
            r#"
            INSERT INTO jobs (
                id, keyword, title, company_name, company_url, licensed_sponsor,
                location, salary, seniority, category, employment_type, url,
                posted_at, fetched_at, raw_json
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, now(), ?
            )
            ON CONFLICT (id) DO UPDATE SET
                keyword = excluded.keyword,
                title = excluded.title,
                company_name = excluded.company_name,
                company_url = excluded.company_url,
                licensed_sponsor = excluded.licensed_sponsor,
                location = excluded.location,
                salary = excluded.salary,
                seniority = excluded.seniority,
                category = excluded.category,
                employment_type = excluded.employment_type,
                url = excluded.url,
                posted_at = excluded.posted_at,
                fetched_at = now(),
                raw_json = excluded.raw_json;
            "#,
            params![
                id,
                keyword,
                title,
                company_name,
                company_url,
                licensed_sponsor,
                location,
                salary,
                seniority,
                category,
                employment_type,
                url,
                posted_at,
                raw_json
            ],
        )?;

        Ok(())
    }

    /// Upserts a slice of jobs and returns the count inserted/updated.
    pub fn upsert_jobs(&self, keyword: &str, jobs: &[serde_json::Value]) -> Result<usize, Box<dyn Error>> {
        let mut count = 0;
        for job in jobs {
            self.upsert_job(keyword, job)?;
            count += 1;
        }
        Ok(count)
    }

    /// Prunes jobs older than the given number of days based on fetched_at (rolling retention).
    /// Returns the number of pruned records.
    pub fn prune_older_than_days(&self, days: i64) -> Result<usize, Box<dyn Error>> {
        let deleted = self.conn.execute(
            r#"
            DELETE FROM jobs
            WHERE fetched_at < (now()::TIMESTAMP - (INTERVAL '1 day' * ?));
            "#,
            params![days],
        )?;
        Ok(deleted)
    }

    /// Gets total count of jobs currently stored.
    pub fn get_total_count(&self) -> Result<usize, Box<dyn Error>> {
        let count: usize = self
            .conn
            .query_row("SELECT COUNT(*) FROM jobs;", [], |row| row.get(0))?;
        Ok(count)
    }

    /// Gets job counts grouped by keyword.
    pub fn get_counts_by_keyword(&self) -> Result<Vec<(String, usize)>, Box<dyn Error>> {
        let mut stmt = self.conn.prepare(
            "SELECT keyword, COUNT(*) FROM jobs GROUP BY keyword ORDER BY COUNT(*) DESC;",
        )?;
        let rows = stmt.query_map([], |row| {
            let kw: String = row.get(0)?;
            let count: usize = row.get(1)?;
            Ok((kw, count))
        })?;

        let mut res = Vec::new();
        for r in rows {
            res.push(r?);
        }
        Ok(res)
    }

    /// Returns a list of recently fetched jobs.
    pub fn get_recent_jobs(&self, limit: usize) -> Result<Vec<JobSummary>, Box<dyn Error>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT id, keyword, title, company_name, location, salary, url, CAST(fetched_at AS VARCHAR)
            FROM jobs
            ORDER BY fetched_at DESC, id DESC
            LIMIT ?;
            "#,
        )?;
        let rows = stmt.query_map(params![limit as i64], |row| {
            Ok(JobSummary {
                id: row.get(0)?,
                keyword: row.get(1)?,
                title: row.get(2)?,
                company_name: row.get(3)?,
                location: row.get(4)?,
                salary: row.get(5)?,
                url: row.get(6)?,
                fetched_at: row.get(7)?,
            })
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

    #[test]
    fn test_job_db_lifecycle() -> Result<(), Box<dyn Error>> {
        let db = JobDb::open_in_memory()?;
        assert_eq!(db.get_total_count()?, 0);

        let sample_job = serde_json::json!({
            "id": 1001,
            "title": "Staff AI Engineer",
            "url": "https://example.com/job/1001",
            "created_at": "2026-11-09T12:00:00Z",
            "salary": "£100,000",
            "location": "London",
            "company": {
                "trading_name": "TestCorp",
                "licensed_sponsor": true,
                "url": "https://testcorp.com"
            },
            "seniority": {"display_name": "Senior"},
            "category": {"display_name": "Data"},
            "type": {"display_name": "Full-time"}
        });

        // Test insert
        db.upsert_job("ai engineer", &sample_job)?;
        assert_eq!(db.get_total_count()?, 1);

        // Test upsert (conflict updates existing record without duplicate)
        db.upsert_job("ai engineer", &sample_job)?;
        assert_eq!(db.get_total_count()?, 1);

        // Test query counts
        let counts = db.get_counts_by_keyword()?;
        assert_eq!(counts.len(), 1);
        assert_eq!(counts[0], ("ai engineer".to_string(), 1));

        // Test recent jobs
        let recent = db.get_recent_jobs(10)?;
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].title, "Staff AI Engineer");
        assert_eq!(recent[0].company_name, "TestCorp");

        // Test pruning older than 7 days (current record should not be pruned)
        let pruned = db.prune_older_than_days(7)?;
        assert_eq!(pruned, 0);
        assert_eq!(db.get_total_count()?, 1);

        // Manually age the record back by 8 days to test rolling pruning
        db.conn.execute(
            "UPDATE jobs SET fetched_at = now()::TIMESTAMP - INTERVAL '8 days' WHERE id = 1001;",
            [],
        )?;
        let pruned = db.prune_older_than_days(7)?;
        assert_eq!(pruned, 1);
        assert_eq!(db.get_total_count()?, 0);

        Ok(())
    }
}
