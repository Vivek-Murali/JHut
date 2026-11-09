# 🌲 Job Hut (JHut)
### UK Tech Talent & Visa Sponsorship Decision Intelligence Engine
> **Powered by [TorTree](https://tortree.com) — AI Studio & Consultancy**  
> *Hardened AI deployments, semantic knowledge graphs & scalable digital architecture.*

---

## 📌 Executive Summary

**Job Hut (JHut)** is a high-performance Rust-powered intelligence platform that automates the extraction, semantic modeling, graph reasoning, and strategic analysis of UK visa-sponsored tech roles across three frontier disciplines:
- **AI Engineer**
- **Data Scientist**
- **Data Engineer**

It ingests live authenticated hiring feeds, maintains a **rolling 7-day retention lakehouse in DuckDB**, projects an in-database **Knowledge Graph (840+ Nodes, 3,400+ Directed Edges)**, evaluates employer culture and Home Office sponsor track records, derives **15 cross-referential market benchmarks**, and delivers an interactive, single-page **Decision Intelligence Dashboard** (`dashboard/index.html`).

---

## 🏗 The 4-Phase Architecture

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                              THE JOB HUT FRAMEWORK                                     │
├─────────────────────┬─────────────────────┬─────────────────────┬──────────────────────┤
│  PHASE 01: EXTRACT  │ PHASE 02: UNDERSTAND│   PHASE 03: PLAN    │   PHASE 04: EXECUTE  │
├─────────────────────┼─────────────────────┼─────────────────────┼──────────────────────┤
│ • Auth session login│ • DuckDB property   │ • Stratified layout │ • Google AI Studio   │
│ • Offset pagination │   graph projection  │                     │   Gemini synthesis   │
│ • Rolling 7-day     │ • Regex skill taxo- │ • Supply vs Demand  │ • 15 derived metrics │
│   intake retention  │   nomy extraction   │   2×2 Matrix        │ • Candidate Arbitrage│
│ • DuckDB upserts    │ • Co-occurrences    │ • Live UK Serp news │   & Moat Calculator  │
└─────────────────────┴─────────────────────┴─────────────────────┴──────────────────────┘
```

---

## 🖥 Visual UI Gists & Dashboard Walkthrough

The dashboard is delivered as a zero-dependency, self-contained single-page application at `dashboard/index.html` styled with the **TorTree brand identity** (*Pine `#14231B`, Evergreen `#1F3D2B`, Moss `#3E8558`, Sage `#8FCBA6`, Mist `#F7FAF6`*).

### 1. Stats Ribbon & Role Selector
```
┌────────────────────┐ ┌────────────────────┐ ┌────────────────────┐ ┌────────────────────┐
│      588 ROLES     │ │   227 SPONSORS     │ │    3,474 EDGES     │ │     91 / 100       │
│ Active Stored Post.│ │ UK Licensed Firms  │ │ Causal Lineage Rel │ │ Avg Role Likeabil. │
└────────────────────┘ └────────────────────┘ └────────────────────┘ └────────────────────┘

[ ALL ROLES OVERVIEW (Active) ]  [ AI ENGINEER ]  [ DATA SCIENTIST ]  [ DATA ENGINEER ]
```

---

### 2. Grey Machines-Style Stratified Decision Blueprint (Frozen Knowledge Graph)
A frozen, 3-stage causal layout rendered on a subtle blueprint dot-grid canvas. Clicking any node card triggers causal tracing—highlighting upstream sponsors and downstream tech moats while dimming unrelated entities to 15% opacity.

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ ◀ STAGE 1: SPONSORS (x=-460) │ STAGE 2: TARGET HUBS (x=0) │ STAGE 3: TECH MOATS (x=+460)▶
├──────────────────────────────┼────────────────────────────┼────────────────────────────┤
│ ┌──────────────────────────┐ │                            │ ┌────────────────────────┐ │
│ │ 🏢 [EMPLOYER]            │ │                            │ │ ⚡ [TECH MOAT]         │ │
│ │ JPMorganChase            │ │   ┌────────────────────┐   │ │ LLMs & Generative AI   │ │
│ │ ★ 30 Sponsoring Roles    │─┼──▶│ 🎯 [TARGET HUB]    │───┼▶│ ★ High Scarcity Asset  │ │
│ └──────────────────────────┘ │   │ AI ENGINEER        │   │ └────────────────────────┘ │
│ ┌──────────────────────────┐ │   │ ★ Top Volume Hub   │   │ ┌────────────────────────┐ │
│ │ 🏢 [EMPLOYER]            │ │   └────────────────────┘   │ │ ⚡ [TECH MOAT]         │ │
│ │ Mistral AI               │─┼──────────────┼─────────────┼▶│ PyTorch & CUDA Kernels │ │
│ │ ★ 9 Sponsoring Roles     │ │              ▼             │ │ ★ +94% Moat Deficit    │ │
│ └──────────────────────────┘ │   ┌────────────────────┐   │ └────────────────────────┘ │
│ ┌──────────────────────────┐ │   │ 🎯 [TARGET HUB]    │   │ ┌────────────────────────┐ │
│ │ 🏢 [EMPLOYER]            │─┼──▶│ DATA ENGINEER      │───┼▶│ ⚡ [TECH MOAT]         │ │
│ │ Wise / Monzo / Arm       │ │   │ ★ Highest Vacancy  │   │ │ Databricks & dbt       │ │
│ └──────────────────────────┘ │   └────────────────────┘   │ └────────────────────────┘ │
└────────────────────────────────────────────────────────────────────────────────────────┘
  ▼ GREY MACHINES DEEP-DIVE DECISION DRAWER
  ┌──────────────────────────────────────────────────────────────────────────────────────┐
  │ [EMPLOYER] MISTRAL AI — 9 Openings · Tier-1 Home Office Licensed                     │
  │ • Strategic Class : Frontier Foundation AI Lab with extreme engineering density.    │
  │ • Lineage Links   : Hiring: AI Engineer · Demanded: PyTorch, CUDA, Distributed Train │
  │ • Supply/Demand   : Top 1% Scarcity Rating — Immediate Skilled Worker sponsorship    │
  └──────────────────────────────────────────────────────────────────────────────────────┘
```

---

### 3. Supply vs. Demand: Macro Scarcity Matrix & Candidate Arbitrage Calculator
Combines macro Home Office shortage dynamics with an interactive user-specific tool:

```
┌───────────────────────────────────────────┐ ┌──────────────────────────────────────────┐
│ 1. MACRO TALENT SCARCITY (2×2 QUADRANT)   │ │ 2. CANDIDATE ARBITRAGE & MOAT CALCULATOR │
├───────────────────────────────────────────┤ ├──────────────────────────────────────────┤
│ Candidate                                 │ │ Select your verified stack:              │
│ Saturation ▲                              │ │ [Python ✓] [AWS ✓] [Databricks ✓] [LLMs] │
│            │  ⚠️ RED OCEAN                │ │ [PyTorch] [SQL] [Kafka] [dbt] [Docker]   │
│      100% ─┼──────────────────────────────┤ │                                          │
│            │  (Basic Python, SQL, Pandas) │ │ ┌────────────┐┌────────────┐┌───────────┐│
│            │                              │ │ │COMPETITION ││SPONSORSHIP ││ MATCHED   ││
│            │         🎯 GOLDEN MOAT       │ │ │  1 : 8.4   ││    89%     ││ 42 Roles  ││
│            │  (LLMs, PyTorch, Databricks, │ │ │Applicants  ││Approval Vel││UK Sponsors││
│        0% ─┼───Kafka Streaming)───────────┤ │ └────────────┘└────────────┘└───────────┘│
│            └────────────────────────────▶ │ │ 💡 MISSING GOLDEN KEY RECOMMENDATION:    │
│            0%         Demand         100% │ │ Adding "PyTorch" or "Databricks" cuts    │
│                                           │ │ candidate competition by 58%.            │
└───────────────────────────────────────────┘ └──────────────────────────────────────────┘
```

---

### 4. Multi-Dimensional Radar Analysis & 15 Derived Benchmarks
Generated via Google AI Studio (Gemini 2.5 Flash) with fallback to local DuckDB deterministic reasoning:

```
┌───────────────────────────────────────────┐ ┌──────────────────────────────────────────┐
│ 15 DERIVED METRICS · RADAR ANALYSIS       │ │ METRIC BREAKDOWN & NATIONAL BENCHMARKS   │
├───────────────────────────────────────────┤ ├──────────────────────────────────────────┤
│                  [Sponsorship]            │ │ Visa Sponsorship Stability  [████████░] 88│
│                       93                  │ │ Salary Competitiveness      [████████░] 88│
│           [Comp] 96   / \   98 [Tech]     │ │ Tech Stack Modernity        [████████░] 92│
│                     /     \               │ │ Hiring Growth Momentum      [████████░] 84│
│         [Moat] 97  |   ★   |  89 [Growth] │ │ Competition Density Index   [███████░░] 78│
│                     \     /               │ │ London Concentration Ratio  [████████░] 87│
│            [Remote]   \ /   91 [Liquidity]│ │ Visa Approval Predictab.    [█████████] 93│
│                       82                  │ │ Skill Scarcity Premium      [████████░] 88│
└───────────────────────────────────────────┘ └──────────────────────────────────────────┘
```

---

### 5. Detailed Employer UK Intelligence Cards
Individual company cards with 5-dimension scoring, Culture & Engineering DNA, core values, interview focus areas, and live UK news signals:

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ JPMorganChase                       Overall Score: 94/100 · AI Engineer Fit: 93%       │
│ Global Investment Banking & FinTech · London (Canary Wharf)                            │
├────────────────────────────────────────────────────────────────────────────────────────┤
│ [Culture: 86]  [Stability: 99]  [Sponsor: 98]  [Stack: 89]  [Compensation: 96]         │
│                                                                                        │
│ • CULTURE DNA        : Disciplined, high-rigor financial engineering with massive comp.│
│ • CORE VALUES        : Extreme Operational Resilience · Meritocratic Accountability    │
│ • WHAT THEY LOOK FOR : Deep low-latency data structures, risk management, and systems. │
│ • INTERVIEW FOCUS    : Algorithms & Concurrency (Java/C++/Python) · Distributed pipes  │
│                                                                                        │
│ 📰 LIVE UK NEWS: "JPMorgan Accelerates UK AI Engineering Hub Expansion in London"       │
│    Financial Times UK · Nov 2026 · [Expansion]                                         │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 🚀 Quick Start

### 1. Prerequisites
- [Rust](https://www.rust-lang.org/) (1.75+)
- Cargo toolchain

### 2. Environment Configuration
Create a `.env` file in the project root:

```env
ROOT_URL=https://app.withpoli.com/api/jobs/search
SIGNIN_URL=https://app.withpoli.com/api/auth/sign-in
USERNAME=your_poli_email@example.com
PWS=your_poli_password
AGENT="Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"

# Optional: Google AI Studio & SerpApi keys
AI_KEY=your_gemini_api_key
SERP=your_serpapi_key
DUCKDB_PATH=jhut.duckdb
```

### 3. Build & Run

#### A. Run full pipeline (login, scrape offset pagination, analyze & generate dashboard):
```bash
cargo run
```

#### B. Regenerate dashboard instantly using cached DuckDB records (`--no-fetch`):
```bash
cargo run -- --no-fetch
```

#### C. Run unit & integration tests:
```bash
cargo test --lib
```

### 4. View Dashboard
Open the generated HTML report in your browser:
```bash
open dashboard/index.html
```

---

## 📂 Project Structure

```
JHut/
├── Cargo.toml                 # Dependencies: duckdb, tokio, reqwest, serde, regex
├── jhut.duckdb                # Local DuckDB database file (588+ jobs, persistent KG)
├── dashboard/
│   └── index.html             # Generated single-page interactive dashboard
├── src/
│   ├── lib.rs                 # Library exports & client initialization
│   ├── db.rs                  # DuckDB schema, upsert, and 7-day retention pruner
│   ├── graph.rs               # Knowledge Graph schema projection & lineage queries
│   ├── analytics.rs           # Regex skill taxonomy, co-occurrence, and time-series
│   ├── enrichment.rs          # 5-dimension company scoring, culture DNA & SerpApi news
│   ├── gemini.rs              # Google AI Studio Gemini engine & deterministic fallback
│   ├── dashboard.rs           # HTML dashboard generator & TorTree template engine
│   └── main.rs                # Orchestration CLI with --no-fetch support
└── README.md
```

---

## 🎨 Design Tokens & Branding

All UI elements adhere to the **TorTree** brand tokens:
- **Mist** (`#F7FAF6`): Canvas and body background
- **Pine** (`#14231B`): Dark primary headers and cards
- **Evergreen** (`#1F3D2B`): Secondary badges and card accents
- **Moss** (`#3E8558`): Primary accent, button fills, and positive indicators
- **Sage** (`#8FCBA6`): Subtle highlight borders and tag backgrounds
- **Slate** (`#5B6660`): Neutral secondary metadata typography
- **Ash** (`#9AA49D`): Muted borders and grid guidelines
- **Display Font**: `Jost` / `Futura`
- **Body Font**: `Inter`

---

## 📄 License & Attribution

Job Hut is built and maintained under the [TorTree](https://tortree.com) Decision Intelligence Practice.  
*Powered by TorTree — AI Studio & Consultancy.*

