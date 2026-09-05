# CHANGELOG

All notable changes to `ai-verification-mcp` are documented here.

---

## [0.9.0] — 2026-09-06 — DAG Structural Hardening + Audit Phase Separation

### Added
- **`audit_phase` field** in `UnifiedAuditInput`: `"plan"` | `"execution"`. Auto-detected when omitted — if `executed_steps` is empty and `planned_tasks` is present, phase defaults to `"plan"`. Exposed in `math_breakdown.audit_phase` output field.
- **Plan phase semantics**: `audit_phase = "plan"` audits DAG structure only; `coverage_ratio = 0` is NOT penalized (it means no execution has happened yet, not failure). Fixes the fundamental semantic confusion between "submitted plan" and "failed execution".
- **`validate_graph()` called in `execute_unified_audit`**: The DAG structural validator (cycle detection + unknown dependency check) is now invoked immediately after building the DAG, before any `record_step()` calls. Unknown dependencies and cycles produce `DAG_STRUCTURAL_ERROR` (Critical → BLOCK).
- **Unknown dependency defense in depth** in `record_step()`: Previously `if let Some(dep_task)` silently ignored missing dependencies at the `record_step` level. Now returns `Err("Unknown dependency...")` explicitly.
- **Duplicate task ID detection** (`DUPLICATE_TASK_ID`): Supplying two tasks with the same `id` now produces a Critical violation → BLOCK. Previously silently overwrote the first definition.
- **Empty/whitespace task ID/name validation** (`INVALID_TASK_ID`, `INVALID_TASK_NAME`): Empty, whitespace-only, or over-length IDs and names are rejected with Critical violations.
- **Invalid `mode` rejection**: Unknown mode strings (e.g. `"MAXIMUM_SECURITY"`) now immediately return `Err(...)` instead of silently falling through to standard behavior.
- **Invalid `audit_phase` rejection**: Unknown phase strings return `Err(...)`.
- **Resource limits**: Hard input size caps prevent DoS / resource exhaustion:
  - `MAX_TASKS = 200`
  - `MAX_REQUIREMENTS = 200`
  - `MAX_EXECUTED_STEPS = 500`
  - `MAX_CODE_BYTES = 512 KB`
  - `MAX_TEXT_BYTES = 64 KB`
  - `MAX_TASK_ID_LEN = 64` chars
  - `MAX_TASK_NAME_LEN = 256` chars
- **`tests/security_invariants.rs`**: New test suite verifying 7 core governance invariants (empty→never ALLOW, unknown dep→BLOCK, cycle→BLOCK, duplicate ID→BLOCK, empty ID→BLOCK, unknown mode→Err, plan phase no coverage penalty, evidence removal cannot improve verdict, adding critical violation cannot improve verdict, resource limits).

### Fixed
- **CI `cargo fmt`** step now runs `cargo fmt --all -- --check` (previously ran `cargo fmt` which formatted code silently without failing).
- **README**: Corrected `verify_agent` description from "Returns 3-tier decision" to **4-tier** (`ALLOW`/`WARN`/`BLOCK`/`INSUFFICIENT_EVIDENCE`).
- **README**: Corrected RFC description from "Validates RFC numbers (1 <= RFC <= 9999)" to "RFC identifiers checked against a curated registry of 70+ published IETF RFCs".
- **README**: Added `audit_phase` field documentation; noted Halstead measures are approximate.

### Changed
- `math_breakdown` output object now includes `"audit_phase"` key showing the resolved phase.
- DAG pillar now inserts unique, valid tasks only (tasks failing validation are excluded from DAG construction).

---

## [0.8.0] — 2026-09-05 — Mandatory Evidence Matrix + Universal Grounding Axiom

### Added
- **Mandatory Evidence Matrix**: `standard` mode now requires `user_requirements` OR `planned_tasks`. An isolated `draft_response`-only payload returns `INSUFFICIENT_EVIDENCE` (not `ALLOW`). `deep` mode requires **both**.
- **Universal Grounding Axiom**: Changed `has_research_deficit` from "all claims failed" to "any claim failed". Any single unverified factual claim → `RESEARCH_DEFICIT` → BLOCK.
- **IETF RFC Registry** (`KNOWN_RFC_REGISTRY`): Replaced permissive `1..=9999` range with a curated list of 70+ real published RFC numbers. Invented RFCs (e.g. `RFC 9999`) are flagged as unverified.
- **Multi-standard per sentence**: `match_structured_standards` now returns `Vec<String>` instead of `Option<String>`, so `"IEEE 754 and ISO/IEC 27001"` in one sentence correctly counts 2 verified citations.
- **Adversarial test suite** (`tests/adversarial_suite.rs`): 4 new tests for partial input gaming, mixed evidence spoofing, uncataloged RFC, and bare acronym markers.

---

## [0.7.0] — 2026-09-04 — Hardened Governance

### Added / Fixed
- Empty input payload → `INSUFFICIENT_EVIDENCE` (not `ALLOW`).
- Authoritative domain validation via hostname boundary (prevents `docs.rs.evil.com` spoofing).
- Nonexistent local file paths no longer count as verified evidence.
- Code blocks (` ``` `) no longer automatically count as evidence.
- DAG dependency failure no longer contaminates execution log.
- Negated resilience markers (`"no retry strategy"`) correctly flagged as defensive deficit.
- Per-sentence evidence binding in ResearchGate.
- Adversarial test suite introduced.

---

## [0.6.0] — Initial Release — Heuristic Verification Layer

- 9 MCP tools (verify_agent, verify_dag, verify_code, verify_diff, verify_text, verify_confidence, verify_research, verify_foresight, verify_constraints) with `math_*` backward-compatible aliases.
- Unified governance gate with 6-pillar weighted scoring.
- Tree-sitter AST integration for Rust, TypeScript, Python, Go, Java, C, C++.
