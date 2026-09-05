# 🛡️ MCP Plugin Math: AI Agent Verification & Governance Layer

[English](README.md) | [Tiếng Việt](README_VI.md)

[![Rust CI/CD](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml/badge.svg)](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Protocol: MCP 2024-11-05](https://img.shields.io/badge/MCP-2024--11--05-brightgreen.svg)](https://modelcontextprotocol.io)
[![Version: 0.5.0](https://img.shields.io/badge/version-0.5.0-orange.svg)](Cargo.toml)

An ultra-high-performance **Model Context Protocol (MCP)** Server written in **Rust** that serves as an **AI Agent Verification & Governance Layer**. It evaluates proposed AI actions, plans, diffs, and draft responses against deterministic mathematical metrics and static analysis, enforcing 3-tier governance decisions (**`ALLOW`**, **`WARN`**, **`BLOCK`**) with machine-readable violation codes and actionable remediation advice.

---

## 🌟 The Philosophy: Deterministic Verification & Guardrails

Large Language Models (LLMs) are inherently probabilistic token predictors. In autonomous multi-agent environments, unconstrained agents can introduce:
1. **Hallucination & Drift**: Fabricating APIs or drifting away from original user intent.
2. **Scope Creep ($W > 0$)**: Executing unapproved, disruptive tasks without justification.
3. **Fluff & Token Inefficiency**: Low information density, conversational filler, or redundant output.
4. **Epistemic Overconfidence**: Making absolute claims ("guaranteed", "100%") without empirical verification.
5. **Research Deficit**: Guessing library versions or specifications instead of grounding in RFCs and docs.
6. **Lazy Planning**: Shallow single-step plans for complex multi-requirement tasks, omitting edge cases.

`mcp-plugin-math` provides a deterministic **Verification & Governance Gate**. Before executing high-impact actions or sending final responses, agents submit their plan and drafts to the governance layer to receive structured feedback:

```
[Agent Proposal] ──► [mcp-plugin-math Governance Gate] ──► ALLOW | WARN | BLOCK
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
- **Hedging & Assertion Density**: Distinguishes scientific humility from evasive dodging.
- **Contradiction Detection**: Identifies opposing statements within the same response ($P \wedge \neg P$).

### 3. Empirical Research Gate
- **Claim Diagnostics**: Analyzes technical claims across categories (performance benchmarks, version specs, API contracts).
- **Evidence Verification**: Verifies presence of grounded evidence (RFC citations, documentation URLs, test logs, source file paths).
- **Research Deficit Detection**: Unverified claims trigger a `RESEARCH_DEFICIT` violation with targeted remediation instructions.

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
- **Dynamic Negation Parsing**: Detects conflicting commitments between constraints and implementation claims.

---

## 🚦 Governance Decision Model: 3-Tier Policy

The unified governance gate (`math_audit_cognition`) evaluates all active pillars and outputs a structured 3-tier decision:

| Decision | Criteria | Agent Behavior |
|:---:|---|---|
| **`ALLOW`** | No critical violations, composite score $\ge 75\%$. | Safe to proceed / deliver response. |
| **`WARN`** | No critical violations, but warnings present (e.g. slight verbosity, minor scope waste, or score $50-75\%$). | Agent may proceed but should review suggestions. |
| **`BLOCK`** | Any critical violation (syntax error, broken DAG order, research deficit, uncalibrated overconfidence, constraint omission) or score $< 50\%$. | Execution halted; agent must apply remediation before retry. |

### Structured Violation Format

```json
{
  "decision": "BLOCK",
  "verdict": "FAIL",
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

## 🛠️ MCP Tools Exposed (9 Tools)

| Tool Name | Type | Description |
|---|---|---|
| **`math_audit_cognition`** | **Unified Gate** | 6-Pillar verification engine. Returns 3-tier decision (`ALLOW`/`WARN`/`BLOCK`), structured violations, and remediation plan. Supports `quick`, `standard`, `deep` modes. |
| **`math_track_dag`** | Granular | Tracks execution on Directed Acyclic Graphs. Detects cycles, dependency errors, and isolates justified exploratory steps. |
| **`math_eval_code`** | Granular | Computes Tree-sitter AST, McCabe Cyclomatic Complexity, Halstead measures, Maintainability Index, and boundary warnings across 7 languages. |
| **`math_eval_diff`** | Granular | Analyzes diffs via LCS, calculating change ratio, complexity deltas ($\Delta M$, $\Delta\text{MI}$), touched functions, and regression risk. |
| **`math_eval_text`** | Granular | Analyzes Shannon entropy, information density, compression ratio, Flesch readability, and conversational padding. |
| **`math_confidence`** | Granular | Epistemic calibration gate. Detects ungrounded overconfidence while whitelisting empirical metrics and specification contracts. |
| **`math_audit_research`** | Granular | Analyzes technical claims (benchmarks, versions, APIs), verifies evidence (RFCs, URLs, test logs), and flags research deficits. |
| **`math_eval_foresight`** | Granular | Evaluates defensive error handling, edge cases, verification strategy, and flags lazy plans. |
| **`math_verify_constraints`** | Granular | Evaluates requirement fulfillment using n-gram Jaccard matching and dynamic negation contradiction analysis. |

---

## 📦 Language Support & Feature Flags

`mcp-plugin-math` uses Cargo feature flags for modular compilation:

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

Pre-compiled standalone binaries for **Linux (x86_64)** and **Windows (x86_64)** are built on every release and commit via [GitHub Actions](https://github.com/Nhan-209/mcp-plugin-math/actions).

### Configuration in MCP Clients

Add to your `claude_desktop_config.json`, Antigravity, or Gemini CLI configuration:

```json
{
  "mcpServers": {
    "math-verifier": {
      "command": "/path/to/mcp-plugin-math",
      "args": []
    }
  }
}
```

Or on Windows:
```json
{
  "mcpServers": {
    "math-verifier": {
      "command": "C:\\path\\to\\mcp-plugin-math.exe",
      "args": []
    }
  }
}
```

---

## 📜 License
Released under the [MIT License](LICENSE).
