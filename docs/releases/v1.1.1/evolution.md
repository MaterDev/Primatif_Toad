# v1.1.1 "Polish & Insights" — Evolution Document

## Vision

v1.1.1 is a **polish and enhancement release** that addresses gaps identified
during v1.1.0 development and adds powerful analytics capabilities. This release
focuses on production readiness, developer experience, and deeper ecosystem
insights.

---

## Core Objectives

1. **Production Infrastructure** — Add CI/CD pipeline for automated testing and
   releases
2. **Rich Analytics** — Provide dependency analysis, velocity metrics, and
   health scoring
3. **Enhanced MCP** — Complete MCP tool surface with CLI bridge and better
   descriptions
4. **Developer Experience** — Add `toad doctor` health checks and comprehensive
   user guide
5. **Dogfooding** — Use Toad's own features to improve Toad development
6. **Content Accuracy** — Update AI skills and documentation for v1.1.0 features

---

## Non-Goals

- Breaking changes to existing APIs
- New major features (save for v1.2.0)
- Performance optimizations (separate track)
- External integrations (GitHub API, etc.)

---

## Release Themes

### Theme 1: Production Readiness

**Problem:** v1.1.0 has no CI/CD, making releases manual and error-prone.

**Solution:**

- GitHub Actions CI pipeline (test, lint, license checks)
- Automated release workflow with multi-platform binaries
- CONTRIBUTING.md for contributors
- Status badges in README

**Impact:** Safe, automated releases with confidence.

### Theme 2: Deeper Insights

**Problem:** Current analytics are basic (disk usage, git status only).

**Solution:**

- Dependency graph analysis
- Development velocity metrics
- Technical debt indicators
- Project health scoring
- Historical trend tracking
- Cross-project pattern analysis
- Submodule health monitoring

**Impact:** Data-driven decisions about where to focus effort.

### Theme 3: Complete MCP Experience

**Problem:** MCP server is functional but incomplete.

**Solution:**

- Enhanced tool descriptions with usage hints
- New tools: `get_atlas`, `get_manifest`, `get_project_context`
- CLI bridge: expose safe CLI commands via MCP
- Better parameter documentation
- Common workflow examples

**Impact:** AI agents can fully leverage Toad's capabilities.

### Theme 4: Developer Experience

**Problem:** Troubleshooting Toad issues requires manual investigation.

**Solution:**

- `toad doctor` command for health checks
- Comprehensive USER_GUIDE.md
- Updated skills with v1.1.0 features
- Better error messages and help text

**Impact:** Faster onboarding and easier troubleshooting.

### Theme 5: Dogfooding

**Problem:** Toad doesn't use its own features for development.

**Solution:**

- Tag Toad's submodules with taxonomy
- Register common workflows as `toad cw` commands
- Configure MCP for Toad development
- Auto-refresh context in git hooks

**Impact:** Validate features in real workflows, identify UX issues.

---

## Technical Design

### Analytics Architecture

**Data Flow:**

````json
Git History + Project Files
    ↓
Analysis Functions (toad-ops)
    ↓
Structured Reports (toad-core types)
    ↓
CLI Formatting (bin/toad) + MCP Tools (bin/toad-mcp)
```json

**Key Types:**

```rust
pub struct DependencyGraph {
    pub nodes: Vec<ProjectNode>,
    pub edges: Vec<DependencyEdge>,
    pub critical_path: Vec<String>,
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

### CI/CD Architecture

**Workflows:**

1. **CI Pipeline** — Runs on every PR and push
   - Test matrix (Ubuntu, macOS)
   - Lint checks (clippy, rustfmt, markdownlint)
   - License boundary verification
   - Build verification

2. **Release Pipeline** — Runs on version tags
   - Multi-platform binary builds
   - Asset uploads to GitHub releases
   - Automated changelog generation

### MCP Enhancements

**New Tools:**

- `get_atlas` — Direct access to ATLAS.json
- `get_manifest` — Direct access to MANIFEST.md
- `get_project_context` — Direct access to CONTEXT.md
- `reveal_projects` — Search functionality
- `get_git_status` — Multi-repo git status
- `sync_registry` — Rebuild cache
- `tag_projects` — Apply taxonomy

**Enhanced Descriptions:** All tools get usage hints, "what comes next"
guidance, and alternative suggestions.

---

## Success Metrics

- ✅ CI pipeline passes on all PRs
- ✅ Release automation works for v1.1.1
- ✅ All 7 analytics types implemented
- ✅ MCP tools complete (20+ tools)
- ✅ `toad doctor` catches common issues
- ✅ USER_GUIDE.md comprehensive
- ✅ Dogfooding workflow validated

---

## Risks & Mitigations

| Risk                  | Mitigation                        |
| --------------------- | --------------------------------- |
| CI takes too long     | Use cargo caching                 |
| Analytics are slow    | Optimize git queries, add caching |
| MCP tools too complex | Keep parameter schemas simple     |
| Scope creep           | Strict adherence to objectives    |

---

## Strategic Positioning

v1.1.1 is a **consolidation release** that:

- Solidifies v1.1.0's foundation
- Adds production-grade infrastructure
- Provides deeper insights for power users
- Sets the stage for v1.2.0 (external integrations)

**After v1.1.1:**

- Toad is production-ready with CI/CD
- Analytics provide actionable insights
- MCP is feature-complete
- Documentation is comprehensive
- Development workflow is optimized

---

## Future Considerations (v1.2.0+)

- GitHub API integration for richer analytics
- Performance profiling and optimization
- Predictive analytics (forecast trends)
- Custom dashboard (web UI)
- Plugin system for custom analytics
- Multi-workspace support
- Cloud sync for context
````
