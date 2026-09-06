# 🛡️ AI Verification MCP: Deterministic Evidence & Policy Enforcement Layer for AI Agents

[English](README.md) | [Tiếng Việt](README_VI.md)

[![Rust CI/CD](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml/badge.svg)](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Protocol: MCP 2026-07-28](https://img.shields.io/badge/MCP-2026--07--28%20%7C%202024--11--05-brightgreen.svg)](https://modelcontextprotocol.io)
[![Version: 0.10.0](https://img.shields.io/badge/version-0.10.0-orange.svg)](Cargo.toml)


An ultra-high-performance **Model Context Protocol (MCP)** Server written in **Rust** that serves as a **Deterministic Evidence & Policy Enforcement Layer for AI Agents**. It evaluates proposed AI actions, plans, diffs, receipts, and draft responses against deterministic signals, AST analysis, and curated provenance registries, enforcing 4-tier governance decisions (**`ALLOW`**, **`WARN`**, **`BLOCK`**, **`INSUFFICIENT_EVIDENCE`**) with machine-readable violation codes and actionable remediation advice.

---

## ⚖️ Trust Model & Epistemic Boundaries

`ai-verification-mcp` operates under a formal, bounded trust model:
$$\text{ALLOW} \equiv \text{No policy violation detected under declared evidence policy}$$
$$\text{ALLOW} \not\equiv \text{Empirical proof that the AI is omniscient or infallible}$$

The verifier eliminates reliance on subjective LLM-as-a-judge patterns by executing **hard deterministic invariants**:
1. **Universal Grounding Axiom**: If an agent makes factual technical claims, every single claim must have verified provenance (`EvidenceClassifier`). Bare code blocks and untrusted URLs are strictly syntactic markers and cannot launder ungrounded claims.
2. **Machine-Verifiable Receipts Layer**: Distinguishes self-attested claims ($S_{\text{claim}}$) from verifiable tool execution receipts ($S_{\text{receipt}}$). Executed steps without matching execution receipts produce `UNATTESTED_EXECUTION_CLAIM`, while non-zero exit codes trigger `FAILED_EXECUTION_RECEIPT` (`BLOCK`).
3. **Non-Authoritative Quick Mode Gate**: `mode: "quick"` is strictly a rapid-loop diagnostic checkpoint returning `decision: "CHECKPOINT_PASS"`, `verdict: "QUICK_PASS"` and `is_delivery_authorized: false`. Authoritative `ALLOW` requires `standard` or `deep` mode. Callers can enforce minimum policies via `min_policy_mode`.
4. **Differentiated Mutation Scope Creep**: Actions with mutating verbs (`send`, `upload`, `post`, `delete`, `drop`, `write`, `alter`, etc.) executed outside the approved plan DAG trigger `UNAPPROVED_MUTATION_SCOPE_CREEP` (Critical $\rightarrow$ `BLOCK`), blocking composite bypass attempts like `send_credentials_to_test`.
5. **Anti-Phase Spoofing Guard**: `audit_phase='plan'` audits proposed plan DAGs without coverage penalty, but strictly forbids execution artifacts. Passing `executed_steps`, `code_snippet`, or completion claims under plan phase triggers `PHASE_SPOOFING` (`BLOCK`). Plan approvals return `verdict='PLAN_APPROVED'` with `is_delivery_authorized: false`.
6. **MCP Revision 2026-07-28 Support**: Supports modern stateless MCP protocol with protocol negotiation and full manifest discovery via `server/discover` while maintaining backward compatibility with `2024-11-05`.

---


## 🌟 The Philosophy: Deterministic Signals Constrain & Audit Agent Behavior

Large Language Models (LLMs) are inherently probabilistic token predictors. While an LLM cannot mathematically guarantee the absence of semantic hallucinations, an independent deterministic verification layer **can** audit and constrain agent reasoning artifacts before execution or user delivery:

1. **Hallucination & Drift**: Fabricating ungrounded APIs or drifting away from original user intent.
2. **Scope Creep ($W > 0$)**: Executing unapproved, disruptive tasks without plan justification.
3. **Fluff & Token Inefficiency**: Low information density, conversational filler, or redundant token waste.
4. **Epistemic Overconfidence**: Making absolute claims ("guaranteed", "100%") without empirical verification.
5. **Research Deficit**: Guessing library versions or specifications instead of grounding in RFCs and docs.
6. **Lazy Planning**: Shallow single-step plans for complex multi-requirement tasks, omitting edge cases.

`ai-verification-mcp` provides a deterministic **Verification & Governance Gate**. Before executing high-impact actions or sending final responses, agents submit their plan, diffs, and drafts to receive structured feedback:

```
[Agent Proposal] ──► [ai-verification-mcp Governance Gate] ──► ALLOW | WARN | BLOCK | INSUFFICIENT_EVIDENCE
                                                                   ▲
                                                        Structured Violations
                                                        & Actionable Remediation
```

---

## 🔬 Core Governance Pillars

### 1. Plan Verification via Directed Acyclic Graphs (DAG) & Justified Discovery
Execution plans are modeled as a DAG $G = (V, E)$.
- **Coverage Ratio**:
  $$C = \frac{|V_{\text{exec}} \cap V_{\text{plan}}|}{|V_{\text{plan}}|}$$
- **Scope Creep & Justified Discovery**:
  Distinguishes unapproved arbitrary actions from **Justified Exploratory Discovery** (`view_file`, `cargo_test`, `grep_search`, `read_url_content`, etc.). Exploratory investigative steps are tracked without false-positive scope creep penalties.
- **Topological Invariants**: Enforces that prerequisite dependencies are satisfied before subsequent tasks run.

### 2. Epistemic Calibration & Context-Aware Confidence
- **Context-Aware Confidence Scoring**: Distinguishes between empirical metric assertions (e.g., `"100% test coverage"`, `"p99 latency < 2ms"`) or specification guarantees (`"guaranteed by RFC 2119"`) and genuine ungrounded epistemic overconfidence.
- **Cautious Negation Whitelist**: Epistemically modest statements (e.g. `"does not guarantee"`, `"does not prove bug-free"`) are recognized as scientific rigor rather than overconfidence.
- **Per-Sentence Evidence Binding**: Prevents **Evidence Laundering** where a single citation at the top whitelists wild subsequent claims.
- **Contradiction Detection**: Identifies opposing statements within the same response ($P \wedge \neg P$).

### 3. Empirical Research Gate & 3-Tier Evidence Lifecycle
- **3-Tier Evidence Verification**: Classifies citations through a deterministic verification pipeline:
  $$\text{Unsupported} \longrightarrow \text{EvidencePresent} \longrightarrow \text{EvidenceVerified}$$
  Validates RFC numbers against `KNOWN_RFC_REGISTRY`, authoritative registries (`docs.rs`, `ietf.org`, `crates.io`, `github.com`), local filesystem paths, and structured standards (`IEEE 754`, `ISO/IEC 27001`). Rejects placeholder URLs (`example.com`) and uncataloged RFC numbers.
- **Claim Diagnostics**: Analyzes technical claims across categories (performance benchmarks, version specs, API contracts).
- **Universal Grounding Enforcement**: Any unverified claim triggers `RESEARCH_DEFICIT` (Critical) and blocks delivery.

### 4. Information Theory & Smart Linguistic Density
- **Shannon Entropy**:
  $$H(X) = -\sum_{x} p(x) \log_2 p(x)$$
- **Information Density**: $D = H(X) \times \text{TTR}$ (Type-Token Ratio) filters low-density token filler.
- **Smart Sentence Splitter (`text_utils.rs`)**: Preserves URLs (`github.com/...`), version tags (`v1.2.3`), floating decimals (`3.14`), and abbreviations (`e.g.`, `i.e.`) without destructive fragmentation.
- **Bilingual AI Filler Detection**: Detects conversational padding in both English and Vietnamese.

### 5. Proactive Foresight & Software Complexity
- **Defensive Design Verification**: Checks for error handling, timeout recovery, fallbacks, and retry mechanisms.
- **Edge Case Coverage**: Validates handling of empty states, boundary values, and resource contention.
- **Plan Diligence**: Detects lazy single-task breakdowns on complex multi-step requirements (`LAZY_PLAN`).
- **McCabe Cyclomatic Complexity**: $M = E - N + 2P \le 15-20$.
- **Maintainability Index (MI)**: Production grade threshold enforcement.
- **Diff & Regression Risk (LCS)**: Analyzes code deltas for complexity spikes and blast radius.

### 6. Semantic Constraint & Dynamic Negation Matching
- **Semantic Constraint Alignment**: Evaluates requirement fulfillment via character n-gram Jaccard matching $J(A, B) = \frac{|A \cap B|}{|A \cup B|}$.
- **Entity Substitution Guard**: Prevents keyword spoofing where core requirement nouns are substituted by arbitrary implementation entities (e.g. replacing `"secrets"` with `"logs"`).
- **Dynamic Negation Parsing**: Detects conflicting commitments between constraints and implementation claims.

---

## 🚦 Governance Decision Model: 4-Tier Policy

The unified governance gate (`verify_agent`) evaluates all active pillars and outputs a structured 4-tier decision:

| Decision | Criteria | Agent Behavior |
|:---:|---|---|
| **`ALLOW`** | All mandatory evidence satisfied, zero critical violations, zero warnings, policy score $\ge 75\%$. | Safe to proceed / deliver response. |
| **`WARN`** | All mandatory evidence satisfied, no critical violations, but warnings present (e.g. slight verbosity, minor scope waste, or score $50-75\%$). | Agent may proceed but should review suggestions. |
| **`BLOCK`** | Any critical violation (syntax error, broken DAG order, ungrounded claim, uncalibrated overconfidence, constraint omission) or score $< 50\%$. | Execution halted; agent must apply remediation before retry. |
| **`INSUFFICIENT_EVIDENCE`** | Empty payload or missing mandatory contract (e.g. isolated draft response in `standard` mode without requirements or plan). | Verification blocked; agent must provide actionable contract specifications or execution trace. |

### Structured Violation Format

```json
{
  "decision": "BLOCK",
  "verdict": "FAIL",
  "policy_score": 42.5,
  "composite_score": 42.5,
  "severity_summary": { "critical": 1, "warning": 1, "info": 0 },
  "violations": [
    {
      "code": "RESEARCH_DEFICIT",
      "message": "Research Deficit: Factual technical assertions made without documentation citations.",
      "severity": "Critical",
      "remediation": "Ground factual claims with official documentation links, RFCs, or benchmark citations."
    }
  ],
  "remediation_plan": [
    "Ground factual claims with official documentation links, RFCs, or benchmark citations."
  ]
}
```

---

## ⚡ Execution Modes (`mode`)

The unified audit supports three execution modes:
- **`quick`**: Ultra-fast light gate (<1ms). Runs constraint verification and epistemic calibration while skipping heavy AST parsing and foresight text evaluation. Ideal for frequent mid-loop checkpoints.
- **`standard`** (default): Full 6-pillar governance audit (<5ms). Balanced rigor for task transitions.
- **`deep`**: Maximum rigor with tightened complexity thresholds (cyclomatic threshold $\le 15$, MI $\ge 65$, complete DAG plan coverage required). Ideal for final PR and release verification.

---

## 🛠️ MCP Tools Exposed (Primary & Legacy Aliases)

All tools feature 100% backward-compatible aliases for legacy `math_*` clients.

| Primary Tool Name | Legacy Alias | Type | Description |
|---|---|---|---|
| **`verify_agent`** | `math_audit_cognition`, `ai_audit_cognition` | **Unified Gate** | 6-Pillar verification engine. Returns **4-tier decision** (`ALLOW`/`WARN`/`BLOCK`/`INSUFFICIENT_EVIDENCE`), structured violations, and remediation plan. Supports `quick`, `standard`, `deep` modes and `audit_phase` (`plan`/`execution`). |
| **`verify_dag`** | `math_track_dag` | Diagnostic | Tracks execution on Directed Acyclic Graphs. Detects cycles, unknown dependency references, duplicate task IDs, and isolates justified exploratory steps. |
| **`verify_code`** | `math_eval_code` | Diagnostic | Computes Tree-sitter AST, McCabe Cyclomatic Complexity, approximate Halstead measures, Maintainability Index, and boundary warnings across 7 languages. |
| **`verify_diff`** | `math_eval_diff` | Diagnostic | Analyzes diffs via LCS, calculating change ratio, complexity deltas ($\Delta M$, $\Delta\text{MI}$), touched functions, and regression risk. |
| **`verify_text`** | `math_eval_text` | Diagnostic | Analyzes Shannon entropy, information density, compression ratio, Flesch readability, and conversational padding. |
| **`verify_confidence`** | `math_confidence` | Diagnostic | Epistemic calibration gate. Detects ungrounded overconfidence while whitelisting empirical metrics and specification contracts. |
| **`verify_research`** | `math_audit_research` | Diagnostic | Analyzes technical claims (benchmarks, versions, APIs). RFC identifiers are checked against a curated registry of 70+ published IETF RFCs (not a permissive numeric range). Authoritative domains validated by hostname boundary. |
| **`verify_foresight`** | `math_eval_foresight` | Diagnostic | Evaluates defensive error handling, edge cases, verification strategy, and flags lazy plans. Note: this is a **prose foresight score**, not an execution artifact verifier. |
| **`verify_constraints`** | `math_verify_constraints` | Diagnostic | Evaluates requirement fulfillment using n-gram Jaccard matching and dynamic negation contradiction analysis. |

---

## 📦 Language Support & Feature Flags

`ai-verification-mcp` uses Cargo feature flags for modular compilation:

| Feature Flag | Languages Supported | Default? |
|---|---|:---:|
| `lang-rust` | Rust | ✅ |
| `lang-typescript` | TypeScript, JavaScript, TSX | ✅ |
| `lang-python` | Python | ✅ |
| `lang-go` | Go | Optional |
| `lang-java` | Java | Optional |
| `lang-c` | C | Optional |
| `lang-cpp` | C++ | Optional |
| `all-languages` | All 7 languages | Optional |

---

## 🚀 Installation & Integration

Pre-compiled standalone binaries for **Linux (x86_64)** and **Windows (x86_64)** are built on every release and commit via [GitHub Actions](https://github.com/Nhan-209/mcp-plugin-math/actions). Both `ai-verification-mcp` and `mcp-plugin-math` binaries are produced identically.

### 🌐 Transport, Protocol Conformance & Trust Boundaries

- **Transport**: Standard Input/Output (`stdio`). The server operates locally as a child process spawned by the client runtime (e.g. Antigravity, Claude Desktop, Cursor).
- **Protocol Compatibility**: Implements a high-performance, stateless JSON-RPC MCP core compatible with selected **2026-07-28** semantics (dynamic protocol negotiation, `server/discover` tool catalog) while maintaining strict backward compatibility with legacy **2024-11-05** clients.
- **Process Isolation & Trust Boundary**: As a local stdio server, process isolation and filesystem permissions are enforced by the operating system. If this verifier is deployed over an untrusted network (e.g. remote HTTP/SSE), an independent authentication, authorization, and TLS termination gateway must be placed in front of the server.

### Configuration in MCP Clients

Add to your `claude_desktop_config.json`, Antigravity, or Gemini CLI configuration:

```json
{
  "mcpServers": {
    "ai-verification-mcp": {
      "command": "/path/to/ai-verification-mcp",
      "args": []
    }
  }
}
```

Or on Windows:
```json
{
  "mcpServers": {
    "ai-verification-mcp": {
      "command": "C:\\path\\to\\ai-verification-mcp.exe",
      "args": []
    }
  }
}
```

---

## 📜 License
Released under the [MIT License](LICENSE).
