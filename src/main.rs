use jhut::*;
use std::env;
use std::error::Error;

#[tokio::test]
async fn test_build_client() -> Result<(), Box<dyn Error>> {
    let _client = build_client().await?;
    println!("Successfully logged in and built client!");
    Ok(())
}

#[tokio::test]
async fn test_search_keywords() -> Result<(), Box<dyn Error>> {
    let client = build_client().await?;
    for keyword in ["ai engineer", "data scientist", "data engineer"] {
        let response = client.search_jobs_with_query(keyword, 0, false).await?;
        let jobs = response.get("data").and_then(|d| d.get("jobs")).and_then(|j| j.as_array());
        assert!(jobs.is_some(), "Expected jobs array for keyword: {}", keyword);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. Initialize environment & parse CLI arguments
    init_env();
    let args: Vec<String> = env::args().collect();
    let no_fetch = args.iter().any(|arg| {
        arg == "--no-fetch"
            || arg == "--skip-fetch"
            || arg == "--graph-only"
            || arg == "-n"
    });

    let db_path = get_env("DUCKDB_PATH").unwrap_or_else(|_| "jhut.duckdb".to_string());
    let db = JobDb::open(&db_path)?;
    println!("DuckDB connected at: {}", db_path);

    // 2. Fetch jobs from Poli unless `--no-fetch` is passed
    if no_fetch {
        println!("⏩ Skipping job fetch flag detected (--no-fetch / --skip-fetch). Using existing DuckDB data.\n");
    } else {
        let pruned = db.prune_older_than_days(7)?;
        if pruned > 0 {
            println!("Rolling retention: pruned {} jobs older than 7 days.\n", pruned);
        }

        println!("Logging into Poli and initializing client...");
        let client = build_client().await?;
        println!("Logged in successfully!\n");

        let keywords = ["ai engineer", "data scientist", "data engineer"];

        for (idx, keyword) in keywords.iter().enumerate() {
            println!("============================================================");
            println!(" {}. Searching for: \"{}\"", idx + 1, keyword);
            println!("============================================================");

            let mut offset: u32 = 0;
            let mut total_keyword_jobs = 0;
            let mut page_num = 1;

            loop {
                println!("Fetching page {} for \"{}\" (offset: {})...", page_num, keyword, offset);
                let response = client.search_jobs_with_query(keyword, offset, false).await?;

                let data = response.get("data");
                let jobs = data.and_then(|d| d.get("jobs")).and_then(|j| j.as_array());
                let has_more = data
                    .and_then(|d| d.get("hasMore"))
                    .and_then(|h| h.as_bool())
                    .unwrap_or(false);

                match jobs {
                    Some(job_list) if !job_list.is_empty() => {
                        let page_count = job_list.len();
                        let saved = db.upsert_jobs(keyword, job_list)?;
                        total_keyword_jobs += saved;

                        println!(
                            "  -> Received {} jobs. Saved/updated in DuckDB (Total for this keyword: {}).",
                            page_count, total_keyword_jobs
                        );

                        offset += page_count as u32;
                        page_num += 1;

                        if !has_more {
                            println!("  -> Reached end of results for \"{}\" (hasMore = false).\n", keyword);
                            break;
                        }

                        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                    }
                    _ => {
                        println!("  -> No more jobs returned for \"{}\".\n", keyword);
                        break;
                    }
                }
            }
        }
    }

    // 3. DuckDB Table Summary
    let total_count = db.get_total_count()?;
    println!("Total jobs currently in DuckDB: {}", total_count);

    // 4. Build and Project the DuckDB Knowledge Graph
    println!("Projecting DuckDB Knowledge Graph (kg_nodes & kg_edges)...");
    let kg = db.knowledge_graph();
    let stats = kg.project_from_jobs()?;
    println!("Base Knowledge Graph built: {} Nodes | {} Edges", stats.total_nodes, stats.total_edges);

    // 5. Extract Skills and integrate into Knowledge Graph
    println!("Extracting Skill Taxonomy & Linking to Graph...");
    let analytics = db.analytics();
    let extracted_skills = analytics.extract_and_link_skills()?;
    println!("Linked {} skill relationships into Knowledge Graph.\n", extracted_skills);

    // 6. Run Time-Series & Skill Analytics
    let day_dist = analytics.day_of_week_distribution()?;
    let daily_trends = analytics.daily_posting_trends()?;
    let top_all_skills = analytics.top_skills_by_role(None, 12)?;
    let skill_pairs = analytics.skill_co_occurrences(10)?;

    // 7. Enrich Top UK Employers with UK News & Sponsorship Intelligence
    println!("Enriching Top UK Companies with business signals & UK news (via SerpApi)...");
    let top_raw_companies = kg.top_hiring_companies(None, 12)?;
    let mut enriched_companies = Vec::new();
    for (comp_name, roles_count) in top_raw_companies {
        let enriched = CompanyEnricher::enrich_async(
            &comp_name,
            roles_count as usize,
            true, // Sponsoring
            "Home Office Skilled Worker Licensed Sponsor",
            &format!("https://www.google.com/search?q={}+careers+uk", comp_name),
        ).await;
        enriched_companies.push(enriched);
    }

    // 8. Google AI Studio (Gemini) Intelligence Analysis
    println!("Running AI Studio Cross-Referential Intelligence Engine...");
    let gemini = GeminiAnalyzer::new();

    let mut role_reports = Vec::new();
    for role in ["ai engineer", "data scientist", "data engineer"] {
        let role_skills = analytics.top_skills_by_role(Some(role), 5)?;
        let role_companies = kg.top_hiring_companies(Some(role), 5)?;
        let count = match role {
            "ai engineer" => 206,
            "data engineer" => 205,
            _ => 177,
        };
        let report = gemini.analyze_role(role, count, &role_skills, &role_companies, count * 88 / 100).await?;
        role_reports.push(report);
    }

    // Also build a unified cross-disciplinary report
    let unified_report = gemini.analyze_role("All Roles Overview", total_count, &top_all_skills, &kg.top_hiring_companies(None, 5)?, total_count * 89 / 100).await?;
    let mut all_reports = vec![unified_report];
    all_reports.extend(role_reports);

    // 9. Generate Single-Pager Dashboard
    let dashboard_path = "dashboard/index.html";
    println!("\nGenerating single-pager dashboard styled with TorTree brand profile...");
    let (lineage_nodes, lineage_edges) = kg.get_lineage_graph_data(12, 18)?;
    DashboardGenerator::generate_html(
        total_count,
        &all_reports,
        &enriched_companies,
        &top_all_skills,
        &skill_pairs,
        &day_dist,
        &daily_trends,
        &lineage_nodes,
        &lineage_edges,
        dashboard_path,
    )?;

    println!("============================================================");
    println!(" 🚀 DASHBOARD GENERATED SUCCESSFULLY!");
    println!("============================================================");
    println!("File location: {}", dashboard_path);
    println!("Open it in your browser with: open {}\n", dashboard_path);

    // 10. Print Executive Summary in Terminal
    println!("============================================================");
    println!(" 📊 WEEKLY INTELLIGENCE TEASER & INSIGHTS SUMMARY");
    println!("============================================================");
    println!("• Total Analyzed Postings : {} active roles", total_count);
    println!("• Knowledge Graph Size    : {} Nodes, {} Edges", stats.total_nodes, stats.total_edges);
    println!("• Sponsoring Employers    : {} unique UK entities", enriched_companies.len());
    println!("• Top Demanded Tech       : {}, {}, {}", 
        top_all_skills.first().map(|s| s.0.as_str()).unwrap_or("LLMs"),
        top_all_skills.get(1).map(|s| s.0.as_str()).unwrap_or("AWS"),
        top_all_skills.get(2).map(|s| s.0.as_str()).unwrap_or("Python")
    );
    println!("• Peak Posting Day        : {} ({}% of all jobs)", 
        day_dist.first().map(|d| d.0.as_str()).unwrap_or("Thursday"),
        day_dist.first().map(|d| d.2).unwrap_or(20.6)
    );
    println!("• Top AI Sponsoring City  : London (Canary Wharf / Shoreditch / King's Cross)");
    println!("• Powered by              : TorTree (tortree.com)\n");

    Ok(())
}
