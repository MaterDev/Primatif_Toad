# Specification: Analytics Enhancements (111-analytics-enhancements)

## Overview

Expand Toad's analytics capabilities to provide deeper insights into ecosystem
health, development velocity, technical debt, and project relationships. These
analytics help developers make data-driven decisions about where to focus
effort.

## Sources

- **Current Analytics:** `toad stats`, `toad status`
- **Toad's Own Analysis:** Using Toad on itself revealed gaps
- **Pre-release Review:** Final v1.1.0 check findings

---

## Problem Statement

Current analytics are basic:

- ✅ Disk usage and bloat detection
- ✅ Git status and activity tiers
- ❌ No dependency analysis
- ❌ No development velocity metrics
- ❌ No technical debt indicators
- ❌ No project relationship mapping
- ❌ No historical trends

**Opportunity:** Rich analytics would help developers understand:

- Which projects are most critical (dependency graph)
- Where technical debt is accumulating
- Development velocity trends
- Project health over time

---

## Goals

1. **Dependency Analytics** — Understand project relationships
2. **Velocity Metrics** — Track development activity
3. **Debt Indicators** — Identify maintenance burden
4. **Historical Trends** — Track changes over time
5. **Health Scoring** — Composite health metrics

---

## Non-Goals

- Real-time monitoring (Toad is a CLI tool, not a service)
- External integrations (GitHub API, etc.)
- Performance profiling (separate concern)

---

## Proposed Analytics

### 1. **Dependency Graph Analysis**

**Command:** `toad analyze deps`

**What It Shows:**

- Which projects depend on which crates
- Dependency depth (how many layers)
- Critical path projects (most depended upon)
- Orphaned projects (no dependents)
- Circular dependencies (if any)

**Output:**

````json
--- DEPENDENCY ANALYSIS ---

Critical Path (Most Depended Upon):
  1. toad-core (7 dependents)
  2. toad-discovery (2 dependents)
  3. toad-git (2 dependents)

Dependency Depth:
  toad-core: 0 (foundation)
  toad-discovery: 1 (depends on toad-core)
  bin/toad: 2 (depends on all crates)

Orphaned Projects:
  - assets (no code dependencies)
  - docs (no code dependencies)

Health: ✅ No circular dependencies detected
```json

**Use Cases:**

- Identify which crates are most critical
- Plan refactoring (know impact radius)
- Understand architecture at a glance

### 2. **Development Velocity Metrics**

**Command:** `toad analyze velocity`

**What It Shows:**

- Commits per project (last 30 days)
- Lines changed per project
- Active contributors per project
- Commit frequency trends

**Output:**

```json
--- DEVELOPMENT VELOCITY (Last 30 Days) ---

Most Active Projects:
  1. bin/toad-mcp (47 commits, +2,341 -892 lines)
  2. toad-core (23 commits, +1,205 -456 lines)
  3. toad-discovery (18 commits, +876 -234 lines)

Stale Projects (No commits in 30 days):
  - toad-scaffold (last commit: 45 days ago)
  - assets (last commit: 120 days ago)

Velocity Trend: ↗️ Increasing (15% more commits than previous 30 days)
```json

**Use Cases:**

- Identify abandoned projects
- Track development momentum
- Plan resource allocation

### 3. **Technical Debt Indicators**

**Command:** `toad analyze debt`

**What It Shows:**

- TODO/FIXME/HACK comments count
- Test coverage gaps
- Outdated dependencies
- Large files (>700 lines, violates Toad's own rule)
- Duplicate code patterns

**Output:**

```json
--- TECHNICAL DEBT ANALYSIS ---

Code Smells:
  ⚠️ 23 TODO comments across 8 files
  ⚠️ 5 FIXME comments in toad-git
  ⚠️ 2 HACK comments in toad-ops

Large Files (>700 lines):
  - bin/toad/src/main.rs (539 lines) ✅
  - crates/toad-discovery/src/scanner.rs (336 lines) ✅
  - bin/toad-mcp/src/server.rs (508 lines) ✅

Test Coverage:
  ✅ toad-core: 85%
  ⚠️ toad-git: 62%
  ❌ toad-scaffold: 45%

Outdated Dependencies:
  - clap: 4.5.0 (latest: 4.5.4)
  - serde: 1.0.195 (latest: 1.0.197)

Debt Score: 6.5/10 (Moderate)
```json

**Use Cases:**

- Prioritize cleanup work
- Track debt accumulation
- Identify refactoring candidates

### 4. **Project Health Scoring**

**Command:** `toad analyze health`

**What It Shows:**

- Composite health score (0-100)
- Health factors breakdown
- Recommendations for improvement

**Scoring Factors:**

- VCS cleanliness (20 points)
- Test coverage (20 points)
- Documentation presence (15 points)
- Recent activity (15 points)
- Dependency health (15 points)
- Code quality (15 points)

**Output:**

```json
--- PROJECT HEALTH SCORES ---

Excellent (90-100):
  ✅ toad-core: 94/100
     - VCS: 20/20 (clean)
     - Tests: 18/20 (85% coverage)
     - Docs: 15/15 (README + inline docs)
     - Activity: 15/15 (active)
     - Deps: 15/15 (up to date)
     - Quality: 11/15 (2 TODOs)

Good (70-89):
  ✅ toad-discovery: 82/100
  ✅ toad-git: 78/100

Needs Attention (50-69):
  ⚠️ toad-scaffold: 65/100
     - VCS: 20/20 (clean)
     - Tests: 9/20 (45% coverage) ⚠️
     - Docs: 12/15 (missing examples)
     - Activity: 8/15 (stale) ⚠️
     - Deps: 12/15 (1 outdated)
     - Quality: 4/15 (many TODOs) ⚠️

Recommendations:
  - Increase test coverage in toad-scaffold
  - Address TODOs in toad-git
  - Update dependencies in toad-ops
```json

**Use Cases:**

- Quick health check
- Identify projects needing attention
- Track improvement over time

### 5. **Historical Trends**

**Command:** `toad analyze trends`

**What It Shows:**

- Health score changes over time
- Disk usage trends
- Activity trends
- Dependency changes

**Output:**

```json
--- ECOSYSTEM TRENDS (Last 90 Days) ---

Health Trend: ↗️ Improving
  - 90 days ago: 72/100
  - 60 days ago: 78/100
  - 30 days ago: 84/100
  - Today: 87/100

Disk Usage Trend: ↗️ Growing
  - 90 days ago: 12.3 GB
  - Today: 17.5 GB (+42%)
  - Bloat ratio: 99% (stable)

Activity Trend: ↗️ Increasing
  - Avg commits/week: 45 (was 32)
  - Active projects: 7/15 (was 5/15)

Dependency Changes:
  - Added: tokio, rmcp, schemars
  - Updated: clap (4.4 → 4.5), serde (1.0.190 → 1.0.195)
  - Removed: wait-timeout
```json

**Use Cases:**

- Track ecosystem evolution
- Identify concerning trends
- Celebrate improvements

### 6. **Cross-Project Patterns**

**Command:** `toad analyze patterns`

**What It Shows:**

- Common code patterns across projects
- Shared dependencies
- Architectural consistency
- Naming conventions

**Output:**

```json
--- CROSS-PROJECT PATTERNS ---

Common Dependencies:
  - serde: 7/7 Rust projects (100%)
  - anyhow: 6/7 Rust projects (86%)
  - clap: 2/7 Rust projects (29%)

Error Handling Patterns:
  ✅ 6/7 use ToadResult consistently
  ⚠️ 1/7 still uses anyhow::Result

Naming Conventions:
  ✅ All crates use toad-* prefix
  ✅ All use snake_case for modules
  ✅ All use CamelCase for types

Architectural Consistency:
  ✅ Clear separation: core → discovery/git/ops → bin
  ✅ No circular dependencies
  ✅ License boundaries respected
```json

**Use Cases:**

- Ensure consistency
- Identify outliers
- Guide new development

### 7. **Submodule Health**

**Command:** `toad analyze submodules`

**What It Shows:**

- Submodule alignment status
- Commit drift analysis
- Update frequency
- Branch consistency

**Output:**

```json
--- SUBMODULE HEALTH ANALYSIS ---

Alignment Status:
  ⚠️ 7/7 submodules show SHA drift
  ✅ All submodules initialized
  ✅ All submodules on correct branch (dev)

Drift Analysis:
  - toad-core: 1 commit ahead
  - toad-discovery: 1 commit ahead
  - toad-git: 1 commit ahead
  (All others similar)

Update Frequency:
  - Last hub update: 2 hours ago
  - Avg submodule lag: 3 hours
  - Recommendation: Run 'toad ggit sync' to align

Branch Consistency:
  ✅ All on 'dev' branch
  ✅ No detached HEADs
```json

**Use Cases:**

- Monitor submodule health
- Identify sync issues
- Plan alignment operations

---

## Implementation Strategy

### Phase 1: Data Collection (Foundation)

Add analytics data structures to `toad-core`:

```rust
pub struct DependencyGraph {
    pub nodes: Vec<ProjectNode>,
    pub edges: Vec<DependencyEdge>,
    pub critical_path: Vec<String>,
}

pub struct VelocityMetrics {
    pub commits_30d: usize,
    pub lines_added: usize,
    pub lines_removed: usize,
    pub contributors: Vec<String>,
    pub trend: Trend,
}

pub struct DebtIndicators {
    pub todo_count: usize,
    pub fixme_count: usize,
    pub large_files: Vec<String>,
    pub test_coverage: f32,
    pub outdated_deps: Vec<String>,
}

pub struct HealthScore {
    pub total: u8,
    pub vcs_score: u8,
    pub test_score: u8,
    pub docs_score: u8,
    pub activity_score: u8,
    pub deps_score: u8,
    pub quality_score: u8,
}
```json

### Phase 2: Analysis Functions (toad-ops)

Add analysis functions to `toad-ops`:

```rust
pub fn analyze_dependencies(workspace: &Workspace) -> ToadResult<DependencyGraph>;
pub fn analyze_velocity(workspace: &Workspace, days: u32) -> ToadResult<VelocityMetrics>;
pub fn analyze_debt(workspace: &Workspace) -> ToadResult<DebtIndicators>;
pub fn calculate_health_score(project: &ProjectDetail) -> ToadResult<HealthScore>;
pub fn analyze_trends(workspace: &Workspace, days: u32) -> ToadResult<TrendReport>;
```json

### Phase 3: CLI Commands (bin/toad)

Add `analyze` subcommand:

```rust
Commands::Analyze {
    #[command(subcommand)]
    subcommand: AnalyzeCommand,
}

enum AnalyzeCommand {
    Deps,
    Velocity { days: Option<u32> },
    Debt,
    Health,
    Trends { days: Option<u32> },
    Patterns,
    Submodules,
}
```json

### Phase 4: MCP Integration

Expose ALL analytics via MCP (7 tools):

```rust
#[tool(description = "Analyze project dependencies and critical path. Shows which projects depend on which crates, identifies critical path, detects circular dependencies.")]
pub async fn analyze_dependencies(...) -> Result<CallToolResult, McpError>

#[tool(description = "Get development velocity metrics for last N days. Shows commits per project, lines changed, active contributors, and stale projects.")]
pub async fn analyze_velocity(...) -> Result<CallToolResult, McpError>

#[tool(description = "Identify technical debt indicators. Shows TODO/FIXME counts, large files, test coverage gaps, outdated dependencies, and debt score.")]
pub async fn analyze_debt(...) -> Result<CallToolResult, McpError>

#[tool(description = "Calculate project health scores (0-100). Composite score based on VCS cleanliness, test coverage, docs, activity, dependencies, and code quality.")]
pub async fn analyze_health(...) -> Result<CallToolResult, McpError>

#[tool(description = "Analyze historical trends over time. Shows health score changes, disk usage trends, activity trends, and dependency evolution.")]
pub async fn analyze_trends(...) -> Result<CallToolResult, McpError>

#[tool(description = "Analyze cross-project patterns. Shows common dependencies, error handling consistency, naming conventions, and architectural patterns.")]
pub async fn analyze_patterns(...) -> Result<CallToolResult, McpError>

#[tool(description = "Analyze submodule health and alignment. Shows alignment status, commit drift, update frequency, and branch consistency.")]
pub async fn analyze_submodules(...) -> Result<CallToolResult, McpError>
```json

**Parameter Schemas:**

```rust
#[derive(Deserialize, JsonSchema)]
pub struct AnalyzeDepsParams {
    /// Optional query to filter projects
    pub query: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct AnalyzeVelocityParams {
    /// Number of days to analyze (default: 30)
    pub days: Option<u32>,
    /// Optional query to filter projects
    pub query: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct AnalyzeDebtParams {
    /// Optional query to filter projects
    pub query: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct AnalyzeHealthParams {
    /// Optional query to filter projects
    pub query: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct AnalyzeTrendsParams {
    /// Number of days to analyze (default: 90)
    pub days: Option<u32>,
}
```json

---

## Success Criteria

- [ ] Dependency graph analysis implemented
- [ ] Velocity metrics calculated from git history
- [ ] Debt indicators detected (TODO/FIXME, coverage, etc.)
- [ ] Health scoring system working
- [ ] Historical trends tracked
- [ ] All analytics exposed via CLI and MCP
- [ ] Documentation updated

---

## Integration Points

- **Depends on:** `toad-core`, `toad-ops`, git history
- **Consumed by:** Developers, AI agents via MCP
- **Testing:** Run on Toad itself to validate

---

## Future Enhancements (v1.2.0+)

- Predictive analytics (forecast trends)
- Comparison with industry benchmarks
- Export to visualization tools
- Integration with GitHub API for richer data
- Custom scoring formulas
````
