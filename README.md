# 🧮 MCP Plugin Math: Formal Metacognition & Anti-Hallucination Engine for AI

[English](README.md) | [Tiếng Việt](README_VI.md)

[![Rust CI/CD](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml/badge.svg)](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Protocol: MCP 2024-11-05](https://img.shields.io/badge/MCP-2024--11--05-brightgreen.svg)](https://modelcontextprotocol.io)
[![Version: 0.3.0](https://img.shields.io/badge/version-0.3.0-orange.svg)](Cargo.toml)

An ultra-high-performance **Model Context Protocol (MCP)** Server written in **Rust** that empowers AI Agents to eliminate hallucinations, prevent scope creep, control verbosity, calibrate confidence, force empirical research, anticipate future failure modes, and ensure production-grade code through **deterministic mathematical proofs and formal software metrics**.

---

## 🌟 The Philosophy: From Stochastic Guessing to Mathematical Proofs

Large Language Models (LLMs) are inherently probabilistic token predictors. Without deterministic grounding, they suffer from:
1. **Hallucination & Drift**: Fabricating facts or deviating from initial user intent.
2. **Scope Creep ($W > 0$)**: Implementing unasked, redundant, or disruptive code.
3. **Fluff & Verbosity**: Wasting tokens with conversational filler, repetitive padding, or low-density prose.
4. **Overconfidence**: Asserting dubious claims with absolute certainty ("guaranteed", "100%") without empirical proof.
5. **Research Deficit**: Guessing library versions, API capabilities, or benchmarks instead of citing verified sources.
6. **Lazy Reactive Planning**: Designing only for the happy path, neglecting error handling, edge cases, or testability.

`mcp-plugin-math` bridges this fundamental gap by providing AI with a deterministic **Mathematical Metacognition Gate**. Before responding or committing code, the AI audits its own actions against formal mathematical models.

---

## 🔬 Mathematical Pillars

### 1. Plan Verification via Directed Acyclic Graphs (DAG)
Every execution plan is modeled as a DAG $G = (V, E)$.
- **Coverage Ratio**:
  $$C = \frac{|V_{\text{exec}} \cap V_{\text{plan}}|}{|V_{\text{plan}}|}$$
- **Waste Metric (Scope Creep)**:
  $$W = |V_{\text{exec}} \setminus V_{\text{plan}}|$$
  Any execution step not present in the approved plan immediately triggers a **Scope Creep Violation** ($W > 0$).
- **Topological Invariants**: Ensures prerequisites are completed strictly before dependent tasks.

### 2. Information Theory & Smart Linguistic Density
- **Shannon Entropy**:
  $$H(X) = -\sum_{x} p(x) \log_2 p(x)$$
- **Information Density**: $D = H(X) \times \text{TTR}$ (Type-Token Ratio). Detects hollow filler tokens and prompt repetition.
- **Kolmogorov Complexity Approximation**: Compression ratio via Gzip filters out redundant, low-entropy fluff.
- **Smart Sentence Splitter (`text_utils.rs`)**: Preserves URLs (`github.com/...`), version strings (`v1.2.3`), floating decimals (`3.14`), and abbreviations (`e.g.`, `i.e.`) without shredding.
- **Bilingual AI Filler Detection**: Detects conversational padding in both English and Vietnamese.

### 3. Epistemic Calibration & Anti-Overconfidence
- **Calibration Index**: Detects and penalizes absolute claims ("guaranteed", "100%", "definitely", "chắc chắn 100%") when made without empirical proof.
- **Hedging & Assertion Density**: Distinguishes between evasive dodging and scientific humility.
- **Verdict Mapping**:
  - `CALIBRATED`: Grounded in facts, confident with evidence.
  - `OVERCONFIDENT`: Absolute claims without proof.
  - `UNDERCONFIDENT`: High technical substance but unnecessarily timid.
  - `EVASIVE`: Empty conversational dodging.

### 4. Empirical Research Gate
- **Evidence Ratio**: $E = \frac{\text{citations}}{\max(\text{factual\_claims}, 1)}$.
- **Research Deficit Detection**: If technical assertions regarding versions, benchmarks, or APIs are made with 0 citations (URLs, RFCs, file paths, test logs), flags `RESEARCH_DEFICIT`.

### 5. Proactive Foresight & Software Complexity
- **Defensive Design**: Checks for error handling, timeout, fallback, and retry strategies.
- **Edge Case Coverage**: Validates handling of empty collections, limits, boundary values, and race conditions.
- **Plan Diligence**: Compares plan depth against requirement complexity to eliminate lazy single-step plans.
- **McCabe Cyclomatic Complexity**: $M = E - N + 2P \le 10$.
- **Maintainability Index (MI)**: $\text{MI} \ge 65$ (Production Grade).
- **Diff & Regression Risk (LCS)**: Analyzes code deltas for complexity jumps and blast radius.

### 6. Set Theory & Dynamic Contradiction Detection
- **Semantic Constraint Matching**: Uses n-gram Jaccard similarity $J(A, B) = \frac{|A \cap B|}{|A \cup B|}$ and token overlap.
- **Dynamic Negation Parsing**: Automatically flags logical contradictions ($P \wedge \neg P \models \bot$).

---

## 🛠️ MCP Tools Exposed (9 Tools)

| Tool Name | Type | Description |
|---|---|---|
| `math_audit_cognition` | **Unified Gate** | 6-Pillar audit (Requirements + DAG + Calibration + Research + Foresight + Code Metrics). Returns `PASS`/`FAIL` and actionable remediation. |
| `math_track_dag` | Granular | Tracks plan execution against a formal DAG. Detects cycles, dependency violations, and scope creep. |
| `math_eval_code` | Granular | Computes AST, McCabe Cyclomatic Complexity, Halstead measures, Maintainability Index, and boundary warnings. |
| `math_eval_diff` | Granular | Analyzes code diffs via LCS, calculating line change ratio, complexity deltas ($\Delta M$, $\Delta\text{MI}$), touched functions, and regression risk. |
| `math_eval_text` | Granular | Analyzes Shannon entropy, information density, compression ratio, Flesch readability, AI filler counts, and optimal length. |
| `math_confidence` | Granular | Epistemic calibration gate. Evaluates overconfidence, unverified certainty, hedging, and self-contradictions. |
| `math_audit_research` | Granular | Forces empirical research. Scans text for technical claims, checks for citations (URLs, RFCs, paths, logs), and flags research deficits. |
| `math_eval_foresight` | Granular | Evaluates defensive error handling, boundary/edge case coverage, verification strategy, and flags lazy shallow plans. |
| `math_verify_constraints` | Granular | Formal set-theory comparison using n-gram Jaccard semantic matching and dynamic negation contradiction detection. |

---

## 📦 Language Support & Feature Flags

`mcp-plugin-math` utilizes Cargo feature flags for modular compilation:

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

To compile with all languages enabled:
```bash
cargo build --release --features all-languages
```

---

## 🚀 Installation & Integration

### Pre-built Binaries (GitHub Actions)
Pre-compiled standalone binaries for **Linux (x86_64)** and **Windows (x86_64)** are automatically built on every release and commit via [GitHub Actions](https://github.com/Nhan-209/mcp-plugin-math/actions).

### Configuring with MCP Clients

Add to your `claude_desktop_config.json` or Antigravity / Gemini CLI MCP configuration:

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

## 🤖 Agentic AI Skill & Rule Integration

This project includes ready-to-use Agent rules and skills:
- **Rule (`rules/math-verification.md`)**: Enforces that AI must invoke `math_audit_cognition` before producing code or completing tasks.
- **Skill (`skills/math-metacognition/SKILL.md`)**: Guides the AI on how to structure DAGs, interpret metric breakdowns, and fix violations.

---

## 📜 License
Released under the [MIT License](LICENSE).
