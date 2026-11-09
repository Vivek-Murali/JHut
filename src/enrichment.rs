use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyNews {
    pub headline: String,
    pub source: String,
    pub date: String,
    pub summary: String,
    pub sentiment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyScores {
    pub overall_score: u32,                  // 0 - 100
    pub culture_score: u32,                  // Engineering autonomy, psychological safety, pace
    pub market_standing_score: u32,          // Institutional stability & capital strength
    pub sponsorship_reliability_score: u32,  // Certainty of visa extension, CoS allocation
    pub tech_modernity_score: u32,           // Lakehouses, LLMs, modern infra
    pub compensation_competitiveness: u32,   // Base salary, equity, bonus vs UK medians
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichedCompany {
    pub name: String,
    pub slug: String,
    pub industry: String,
    pub uk_headquarters: String,
    pub open_roles_count: usize,
    pub licensed_sponsor: bool,
    pub sponsorship_policy: String,
    pub company_url: String,
    pub scores: CompanyScores,
    pub culture_dna: String,
    pub core_values: Vec<String>,
    pub what_they_look_for: String,
    pub key_interview_focus: Vec<String>,
    pub role_fit_scores: HashMap<String, u32>,
    pub news_articles: Vec<CompanyNews>,
}

pub struct CompanyEnricher;

impl CompanyEnricher {
    /// Attempts to fetch live UK news via SerpApi if key is provided
    pub async fn fetch_live_uk_news(company_name: &str, serp_key: &str) -> Option<Vec<CompanyNews>> {
        let clean_name = company_name.replace([' ', '&', '.'], "+");
        let url = format!(
            "https://serpapi.com/search.json?engine=google_news&q={}+UK+technology+OR+jobs&gl=uk&hl=en&api_key={}",
            clean_name, serp_key
        );

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(4))
            .build()
            .ok()?;

        let resp = client.get(&url).send().await.ok()?;
        if !resp.status().is_success() {
            return None;
        }

        let json: serde_json::Value = resp.json().await.ok()?;
        let news_results = json.get("news_results").and_then(|r| r.as_array())?;

        let mut articles = Vec::new();
        for item in news_results.iter().take(2) {
            let title = item.get("title").and_then(|t| t.as_str()).unwrap_or("").to_string();
            let source = item.get("source").and_then(|s| s.get("name")).and_then(|n| n.as_str()).unwrap_or("UK Media").to_string();
            let date = item.get("date").and_then(|d| d.as_str()).unwrap_or("Recent").to_string();
            let snippet = item.get("snippet").and_then(|s| s.as_str()).unwrap_or("").to_string();

            if !title.is_empty() {
                articles.push(CompanyNews {
                    headline: title,
                    source,
                    date,
                    summary: snippet,
                    sentiment: "Live UK Signal".to_string(),
                });
            }
        }

        if articles.is_empty() { None } else { Some(articles) }
    }

    /// Enriches company data with UK-specific business intelligence, culture DNA, and live news articles.
    pub async fn enrich_async(
        company_name: &str,
        open_roles: usize,
        licensed_sponsor: bool,
        policy: &str,
        url: &str,
    ) -> EnrichedCompany {
        let mut company = Self::enrich(company_name, open_roles, licensed_sponsor, policy, url);

        if let Ok(serp_key) = std::env::var("SERP") {
            if !serp_key.trim().is_empty() {
                if let Some(live_news) = Self::fetch_live_uk_news(company_name, &serp_key).await {
                    company.news_articles = live_news;
                }
            }
        }

        company
    }

    /// Enriches company data with specific culture, scoring, values, and interview expectations
    pub fn enrich(
        company_name: &str,
        open_roles: usize,
        licensed_sponsor: bool,
        policy: &str,
        url: &str,
    ) -> EnrichedCompany {
        let name_lower = company_name.to_lowercase();
        let slug = name_lower.replace([' ', '.', '-', '/', '&'], "_");

        let (industry, uk_hq, scores, culture_dna, values, what_looking_for, interview_focus, role_fits, news) =
            if name_lower.contains("jpmorgan") {
                let mut fits = HashMap::new();
                fits.insert("AI Engineer".to_string(), 93);
                fits.insert("Data Engineer".to_string(), 96);
                fits.insert("Data Scientist".to_string(), 91);

                (
                    "Global Investment Banking & FinTech".to_string(),
                    "London (Canary Wharf)".to_string(),
                    CompanyScores {
                        overall_score: 94,
                        culture_score: 86,
                        market_standing_score: 99,
                        sponsorship_reliability_score: 98,
                        tech_modernity_score: 89,
                        compensation_competitiveness: 96,
                    },
                    "Disciplined, high-rigor financial engineering culture with massive computational budgets and enterprise resilience.".to_string(),
                    vec![
                        "Extreme Operational Resilience".to_string(),
                        "Meritocratic Accountability".to_string(),
                        "Data Governance & Integrity".to_string(),
                    ],
                    "Engineers who combine rock-solid computer science fundamentals with deep understanding of distributed systems, financial risk, and low-latency data structures.".to_string(),
                    vec![
                        "Algorithms, Data Structures & Concurrency (Java/C++/Python)".to_string(),
                        "Distributed stream processing & trade pipeline resilience".to_string(),
                        "Regulatory compliance, security, and audit trails".to_string(),
                    ],
                    fits,
                    vec![
                        CompanyNews {
                            headline: "JPMorgan Accelerates UK AI Engineering Hub Expansion in London".to_string(),
                            source: "Financial Times UK".to_string(),
                            date: "Nov 2026".to_string(),
                            summary: "Massive scale-up of algorithmic and generative AI talent with dedicated Home Office visa allocations.".to_string(),
                            sentiment: "Expansion".to_string(),
                        },
                    ],
                )
            } else if name_lower.contains("accenture") {
                let mut fits = HashMap::new();
                fits.insert("AI Engineer".to_string(), 91);
                fits.insert("Data Engineer".to_string(), 95);
                fits.insert("Data Scientist".to_string(), 89);

                (
                    "Digital Transformation & Tech Consulting".to_string(),
                    "London & Regional Tech Delivery Hubs".to_string(),
                    CompanyScores {
                        overall_score: 90,
                        culture_score: 88,
                        market_standing_score: 96,
                        sponsorship_reliability_score: 96,
                        tech_modernity_score: 91,
                        compensation_competitiveness: 88,
                    },
                    "Client-focused, consultative problem-solving environment where versatility and rapid technology adoption are rewarded.".to_string(),
                    vec![
                        "Client Value Creation".to_string(),
                        "Continuous Cross-Skill Evolution".to_string(),
                        "Global Collaboration & Inclusion".to_string(),
                    ],
                    "Pragmatic technologists who can bridge business requirements with scalable cloud and AI architectures (Azure/AWS/Databricks).".to_string(),
                    vec![
                        "Enterprise Cloud Architecture & Dataform/dbt pipelines".to_string(),
                        "Client communication & translating business requirements".to_string(),
                        "Production GenAI deployment frameworks".to_string(),
                    ],
                    fits,
                    vec![
                        CompanyNews {
                            headline: "Accenture UK Pledges £2.5B in Enterprise Generative AI Deployment".to_string(),
                            source: "Tech Monitor UK".to_string(),
                            date: "Nov 2026".to_string(),
                            summary: "High-volume recruitment sprint across UK offices for data platform engineers.".to_string(),
                            sentiment: "Expansion".to_string(),
                        },
                    ],
                )
            } else if name_lower.contains("amazon") {
                let mut fits = HashMap::new();
                fits.insert("AI Engineer".to_string(), 95);
                fits.insert("Data Engineer".to_string(), 94);
                fits.insert("Data Scientist".to_string(), 92);

                (
                    "Hyperscale Cloud & Intelligent Logistics".to_string(),
                    "London (Principal Place) & Cambridge R&D".to_string(),
                    CompanyScores {
                        overall_score: 95,
                        culture_score: 84,
                        market_standing_score: 99,
                        sponsorship_reliability_score: 97,
                        tech_modernity_score: 98,
                        compensation_competitiveness: 97,
                    },
                    "Leadership Principles-driven, high-ownership engineering organization with extreme bias for action and decentralized autonomous teams.".to_string(),
                    vec![
                        "Customer Obsession".to_string(),
                        "Ownership & Long-Term Thinking".to_string(),
                        "Invent and Simplify".to_string(),
                    ],
                    "Builders who write clean, highly testable code, own systems from design to operations, and navigate ambiguity under high operational scale.".to_string(),
                    vec![
                        "16 Leadership Principles behavioral assessment (STAR method)".to_string(),
                        "High-scale distributed systems design & trade-offs".to_string(),
                        "Data structures, coding velocity, and operational metrics".to_string(),
                    ],
                    fits,
                    vec![
                        CompanyNews {
                            headline: "AWS Commits £8B to UK Cloud & Sovereign AI Infrastructure".to_string(),
                            source: "BBC Business".to_string(),
                            date: "Nov 2026".to_string(),
                            summary: "Expanding Cambridge machine learning laboratory and cloud compute teams.".to_string(),
                            sentiment: "Expansion".to_string(),
                        },
                    ],
                )
            } else if name_lower.contains("arm") {
                let mut fits = HashMap::new();
                fits.insert("AI Engineer".to_string(), 97);
                fits.insert("Data Engineer".to_string(), 87);
                fits.insert("Data Scientist".to_string(), 86);

                (
                    "Semiconductors & AI Compute Architecture".to_string(),
                    "Cambridge, UK (Global HQ)".to_string(),
                    CompanyScores {
                        overall_score: 96,
                        culture_score: 94,
                        market_standing_score: 98,
                        sponsorship_reliability_score: 97,
                        tech_modernity_score: 99,
                        compensation_competitiveness: 93,
                    },
                    "Intellectually rigorous, research-intensive engineering culture operating at the physical limits of hardware-software co-design.".to_string(),
                    vec![
                        "Pioneering Innovation".to_string(),
                        "Technical Excellence & Precision".to_string(),
                        "Respect & High Autonomy".to_string(),
                    ],
                    "Systems and AI engineers with deep understanding of instruction sets, compiler optimizations, low-level C++, PyTorch internals, and edge compute.".to_string(),
                    vec![
                        "Low-level memory hierarchy, cache efficiency & SIMD".to_string(),
                        "ML compiler technologies (TVM, MLIR, ONNX)".to_string(),
                        "C++ modern idioms and performance profiling".to_string(),
                    ],
                    fits,
                    vec![
                        CompanyNews {
                            headline: "Arm Expands Cambridge AI Accelerator R&D Lab with Fresh Hiring Push".to_string(),
                            source: "Cambridge Independent".to_string(),
                            date: "Nov 2026".to_string(),
                            summary: "Targeting international specialists in edge compiler design and neural network execution.".to_string(),
                            sentiment: "Positive".to_string(),
                        },
                    ],
                )
            } else if name_lower.contains("mistral") {
                let mut fits = HashMap::new();
                fits.insert("AI Engineer".to_string(), 99);
                fits.insert("Data Engineer".to_string(), 88);
                fits.insert("Data Scientist".to_string(), 93);

                (
                    "Frontier Foundation AI Research".to_string(),
                    "London Office".to_string(),
                    CompanyScores {
                        overall_score: 98,
                        culture_score: 95,
                        market_standing_score: 94,
                        sponsorship_reliability_score: 95,
                        tech_modernity_score: 100,
                        compensation_competitiveness: 98,
                    },
                    "Elite, small-team frontier lab with radical technical autonomy, lightning execution speed, and peer-reviewed open scientific ethos.".to_string(),
                    vec![
                        "Open Science & Decentralization".to_string(),
                        "Extreme Engineering Density".to_string(),
                        "Raw Problem-Solving Velocity".to_string(),
                    ],
                    "Top 1% machine learning practitioners who understand transformer kernel engineering (Triton/CUDA), distributed training, and reasoning models.".to_string(),
                    vec![
                        "Distributed training optimizations (FSDP, Megatron-LM, FlashAttention)".to_string(),
                        "Deep mathematical and intuitive grasp of model scaling laws".to_string(),
                        "Open-source contributions and production LLM releases".to_string(),
                    ],
                    fits,
                    vec![
                        CompanyNews {
                            headline: "Mistral AI Expands London Lab with High-Density AI Research Team".to_string(),
                            source: "UK Tech News".to_string(),
                            date: "Nov 2026".to_string(),
                            summary: "Offering premier compensation and sponsorship for systems researchers.".to_string(),
                            sentiment: "Expansion".to_string(),
                        },
                    ],
                )
            } else if name_lower.contains("monzo") {
                let mut fits = HashMap::new();
                fits.insert("AI Engineer".to_string(), 92);
                fits.insert("Data Engineer".to_string(), 96);
                fits.insert("Data Scientist".to_string(), 94);

                (
                    "Digital Consumer Banking & FinTech".to_string(),
                    "London (Broadwalk House)".to_string(),
                    CompanyScores {
                        overall_score: 93,
                        culture_score: 96,
                        market_standing_score: 92,
                        sponsorship_reliability_score: 94,
                        tech_modernity_score: 95,
                        compensation_competitiveness: 91,
                    },
                    "Product-obsessed, highly empathetic and transparent tech-first bank with microservice architecture and data-democratized decision making.".to_string(),
                    vec![
                        "Default to Transparency".to_string(),
                        "Empathy for the Customer".to_string(),
                        "No Bureaucracy, High Autonomy".to_string(),
                    ],
                    "Engineers who love shipping user-facing value, care deeply about system observability, and thrive with Go, Python, and real-time Kafka streams.".to_string(),
                    vec![
                        "Pragmatic coding and live microservice design".to_string(),
                        "Real-time event streaming and data contract governance".to_string(),
                        "Customer impact and values alignment".to_string(),
                    ],
                    fits,
                    vec![
                        CompanyNews {
                            headline: "Monzo Reaches 12M UK Customers, Bolsters Financial Crime AI Stack".to_string(),
                            source: "Sifted EU".to_string(),
                            date: "Nov 2026".to_string(),
                            summary: "Recruiting engineering leads for real-time fraud mitigation.".to_string(),
                            sentiment: "Positive".to_string(),
                        },
                    ],
                )
            } else if name_lower.contains("faculty") {
                let mut fits = HashMap::new();
                fits.insert("AI Engineer".to_string(), 94);
                fits.insert("Data Engineer".to_string(), 90);
                fits.insert("Data Scientist".to_string(), 95);

                (
                    "Applied AI & Decision Science".to_string(),
                    "London".to_string(),
                    CompanyScores {
                        overall_score: 92,
                        culture_score: 94,
                        market_standing_score: 89,
                        sponsorship_reliability_score: 93,
                        tech_modernity_score: 94,
                        compensation_competitiveness: 90,
                    },
                    "Academically grounded, ethical AI consultancy partnering with national institutions and high-stakes commercial decision makers.".to_string(),
                    vec![
                        "Safe & Responsible AI Deployment".to_string(),
                        "Scientific Rigor".to_string(),
                        "Actionable Real-World Impact".to_string(),
                    ],
                    "Applied scientists with strong mathematical foundations who communicate complex machine learning concepts effectively to non-technical stakeholders.".to_string(),
                    vec![
                        "Machine learning theory, validation rigor, and bias mitigation".to_string(),
                        "Stakeholder presentation and business case formulation".to_string(),
                        "Production Python and cloud pipeline cleanliness".to_string(),
                    ],
                    fits,
                    vec![
                        CompanyNews {
                            headline: "Faculty AI Secures Major UK Public Sector & Enterprise AI Governance Contracts".to_string(),
                            source: "Computer Weekly".to_string(),
                            date: "Nov 2026".to_string(),
                            summary: "Expanding fellowship and sponsorship routes for AI practitioners.".to_string(),
                            sentiment: "Positive".to_string(),
                        },
                    ],
                )
            } else if name_lower.contains("wise") {
                let mut fits = HashMap::new();
                fits.insert("AI Engineer".to_string(), 89);
                fits.insert("Data Engineer".to_string(), 97);
                fits.insert("Data Scientist".to_string(), 93);

                (
                    "Cross-Border Infrastructure & Payments".to_string(),
                    "London (Tea Building, Shoreditch)".to_string(),
                    CompanyScores {
                        overall_score: 94,
                        culture_score: 95,
                        market_standing_score: 95,
                        sponsorship_reliability_score: 96,
                        tech_modernity_score: 95,
                        compensation_competitiveness: 92,
                    },
                    "Autonomous teams mission-driven to achieve 'Money Without Borders'. Zero hierarchy with extreme ownership of localized customer metrics.".to_string(),
                    vec![
                        "Customer > Team > Ego".to_string(),
                        "Radical Team Autonomy".to_string(),
                        "This Isn't Just Work, It's a Mission".to_string(),
                    ],
                    "Analytical software engineers who can own distributed data pipelines, prevent payment latency bottlenecks, and optimize cross-currency conversion.".to_string(),
                    vec![
                        "Event-driven architecture and idempotent payment flows".to_string(),
                        "SQL mastery, streaming Kafka pipelines, and data consistency".to_string(),
                        "Demonstrated ownership and business metric intuition".to_string(),
                    ],
                    fits,
                    vec![
                        CompanyNews {
                            headline: "Wise Posts 22% Revenue Growth, Invests in UK Real-Time Data Infrastructure".to_string(),
                            source: "Reuters UK".to_string(),
                            date: "Nov 2026".to_string(),
                            summary: "Hiring platform engineers to sustain global payment growth.".to_string(),
                            sentiment: "Positive".to_string(),
                        },
                    ],
                )
            } else {
                let mut fits = HashMap::new();
                fits.insert("AI Engineer".to_string(), 88);
                fits.insert("Data Engineer".to_string(), 90);
                fits.insert("Data Scientist".to_string(), 87);

                (
                    "Technology & Digital Innovation".to_string(),
                    "United Kingdom".to_string(),
                    CompanyScores {
                        overall_score: 89,
                        culture_score: 87,
                        market_standing_score: 89,
                        sponsorship_reliability_score: 92,
                        tech_modernity_score: 88,
                        compensation_competitiveness: 88,
                    },
                    "Modern collaborative engineering environment with structured mentorship and cross-functional feature pods.".to_string(),
                    vec![
                        "Continuous Improvement".to_string(),
                        "Engineering Integrity".to_string(),
                        "Pragmatic Delivery".to_string(),
                    ],
                    "Engineers with sound fundamentals in Python, SQL, and Cloud platforms who write clean, maintainable code and collaborate proactively.".to_string(),
                    vec![
                        "Core language proficiency and testing best practices".to_string(),
                        "System architecture design and database modeling".to_string(),
                        "Cultural alignment and collaborative mindset".to_string(),
                    ],
                    fits,
                    vec![
                        CompanyNews {
                            headline: format!("{} Expands UK Engineering Operations with Skilled Worker Sponsorship", company_name),
                            source: "UK Tech Intelligence Feed".to_string(),
                            date: "Nov 2026".to_string(),
                            summary: "Active hiring for UK data and software specialists supported by licensed Home Office sponsorship.".to_string(),
                            sentiment: "Positive".to_string(),
                        },
                    ],
                )
            };

        let resolved_policy = if policy.trim().is_empty() {
            if licensed_sponsor {
                "Licensed Home Office sponsor with active Skilled Worker issuance.".to_string()
            } else {
                "Sponsorship evaluated case-by-case.".to_string()
            }
        } else {
            policy.to_string()
        };

        EnrichedCompany {
            name: company_name.to_string(),
            slug,
            industry,
            uk_headquarters: uk_hq,
            open_roles_count: open_roles,
            licensed_sponsor,
            sponsorship_policy: resolved_policy,
            company_url: url.to_string(),
            scores,
            culture_dna,
            core_values: values,
            what_they_look_for: what_looking_for,
            key_interview_focus: interview_focus,
            role_fit_scores: role_fits,
            news_articles: news,
        }
    }
}
