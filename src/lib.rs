use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

pub mod db;
pub mod graph;
pub mod analytics;
pub mod enrichment;
pub mod gemini;
pub mod dashboard;

pub use db::{JobDb, JobSummary};
pub use graph::{KnowledgeGraph, GraphStats};
pub use analytics::Analytics;
pub use enrichment::{CompanyEnricher, EnrichedCompany, CompanyNews};
pub use gemini::{GeminiAnalyzer, RoleAnalysisReport, DerivedMetric};
pub use dashboard::DashboardGenerator;

pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
pub const DEFAULT_SIGNIN_URL: &str = "https://app.withpoli.com/api/auth/sign-in";

#[derive(Serialize)]
struct SignInPayload<'a> {
    #[serde(rename = "emailAddress")]
    email_address: &'a str,
    password: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct AuthResponse {
    pub data: Option<serde_json::Value>,
    pub error: Option<AuthError>,
}

#[derive(Deserialize, Debug)]
pub struct AuthError {
    pub message: String,
}

pub struct Client {
    pub base_url: String,
    pub http_client: reqwest::Client,
}

#[derive(Deserialize, Debug)]
pub struct Keywords {
    pub keywords: Vec<String>,
}

impl Keywords {
    pub fn new() -> Self {
        Self {
            keywords: Vec::new(),
        }
    }
}



impl Client {
    pub fn new(base_url: String, agent: &str) -> Result<Self, Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(agent)?);
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let http_client = reqwest::Client::builder()
            .cookie_store(true) // Automatically stores & forwards session cookies
            .default_headers(headers)
            .build()?;

        Ok(Self {
            base_url,
            http_client,
        })
    }

    /// Logs into app.withpoli.com using email and password.
    /// The session cookie (`sb-*-auth-token`) is automatically stored in `http_client`.
    pub async fn login(&self, email: &str, password: &str, signin_url: &str) -> Result<(), Box<dyn Error>> {
        let payload = SignInPayload {
            email_address: email,
            password,
        };

        let response = self
            .http_client
            .post(signin_url)
            .json(&payload)
            .send()
            .await?;

        let status = response.status();
        let auth_resp: AuthResponse = response.json().await?;

        if !status.is_success() {
            if let Some(err) = auth_resp.error {
                return Err(format!("Login failed: {}", err.message).into());
            }
            return Err(format!("Login failed with HTTP status {}", status).into());
        }

        if let Some(err) = auth_resp.error {
            return Err(format!("Login failed: {}", err.message).into());
        }

        Ok(())
    }

    /// Sends a JSON POST request to the base_url (e.g. /api/jobs/search)
    pub async fn send_json(&self) -> Result<serde_json::Value, Box<dyn Error>> {
        let empty_payload = serde_json::json!({});
        let response = self
            .http_client
            .post(&self.base_url)
            .json(&empty_payload)
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;

        Ok(response)
    }

    /// Search jobs with custom search payload
    pub async fn search_jobs(&self, payload: &serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {
        let response = self
            .http_client
            .post(&self.base_url)
            .json(payload)
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;

        Ok(response)
    }

    /// Search jobs with a specific keyword query, offset, and preferences flag
    pub async fn search_jobs_with_query(&self, query: &str, offset: u32, use_preferences: bool) -> Result<serde_json::Value, Box<dyn Error>> {
        let payload = serde_json::json!({
            "query": query,
            "offset": offset,
            "usePreferences": use_preferences
        });
        self.search_jobs(&payload).await
    }

    /// Search jobs with default offset (0) and preferences disabled
    pub async fn search_jobs_by_keyword(&self, query: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        self.search_jobs_with_query(query, 0, false).await
    }
}

pub fn init_env() {
    dotenvy::dotenv().ok();
}

pub fn get_env(key: &str) -> Result<String, Box<dyn Error>> {
    env::var(key).map_err(|e| format!("Missing environment variable '{}': {}", key, e).into())
}

/// Builds the client, loads credentials from `.env`, and authenticates against Poli.
pub async fn build_client() -> Result<Client, Box<dyn Error>> {
    init_env();
    let base_url = get_env("ROOT_URL")?;
    let username = get_env("USERNAME")?;
    let password = get_env("PWS")?;
    let signin_url = get_env("SIGNIN_URL").unwrap_or_else(|_| DEFAULT_SIGNIN_URL.to_string());
    let agent = get_env("AGENT").unwrap_or_else(|_| DEFAULT_USER_AGENT.to_string());

    let client = Client::new(base_url, &agent)?;
    client.login(&username, &password, &signin_url).await?;
    Ok(client)
}
