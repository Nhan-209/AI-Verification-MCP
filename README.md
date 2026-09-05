# 🧮 MCP Plugin Math: Formal Metacognition & Anti-Hallucination Engine for AI

[English](README.md) | [Tiếng Việt](README_VI.md)

[![Rust CI/CD](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml/badge.svg)](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Protocol: MCP 2024-11-05](https://img.shields.io/badge/MCP-2024--11--05-brightgreen.svg)](https://modelcontextprotocol.io)
[![Version: 0.2.0](https://img.shields.io/badge/version-0.2.0-orange.svg)](Cargo.toml)

An ultra-high-performance **Model Context Protocol (MCP)** Server written in **Rust** that empowers AI Agents to eliminate hallucinations, prevent scope creep, control verbosity, assess confidence, evaluate diff regression risks, and ensure production-grade code through **deterministic mathematical proofs and formal software metrics**.

---

## 🌟 The Philosophy: From Stochastic Guessing to Mathematical Proofs

Large Language Models (LLMs) are inherently probabilistic token predictors. Without deterministic grounding, they suffer from:
1. **Hallucination & Drift**: Fabricating facts or deviating from initial intentions.
2. **Scope Creep ($W > 0$)**: Implementing unasked, redundant, or disruptive code.
3. **Fluff & Verbosity**: Wasting tokens with conversational filler, repetitive padding, or low-density prose.
4. **False Confidence**: Asserting dubious assumptions without evidence or hedging excessively.
5. **Code Regressions**: Introducing hidden complexity jumps or breaking invariants during code modifications.

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

### 2. Information Theory & Linguistic Density
- **Shannon Entropy**:
  $$H(X) = -\sum_{x} p(x) \log_2 p(x)$$
- **Information Density**: $D = H(X) \times \text{TTR}$ (Type-Token Ratio). Detects hollow filler tokens and prompt repetition.
- **Kolmogorov Complexity Approximation**: Compression ratio via Gzip/Zlib filters out redundant, low-entropy fluff.
- **AI Filler Detection & Optimal Length**: Detects conversational padding ("as an ai", "i'd be happy to") and computes optimal response length based on entropy saturation.
- **Readability Indices**: Flesch Reading Ease & Gunning Fog Index ensure outputs remain clear, engaging, and neither dry nor convoluted.

### 3. Metacognitive Confidence & Self-Consistency
- **Hedging Ratio**: Measures frequencies of uncertainty markers (*maybe*, *probably*, *I think*, *might*).
- **Assertion Density**: $A = 1 - H$, capturing decisive and grounded statements.
- **Specificity Index**: Measures concrete empirical references (code blocks, file paths, numbers, URLs).
- **Self-Contradiction Analysis**: Automatically detects conflicting assertions within responses.

### 4. Software Complexity, AST & Diff Analysis
- **McCabe Cyclomatic Complexity**: $M = E - N + 2P$. Flags functions with high decision density.
- **Halstead Metrics**: Computes Program Volume ($V$), Difficulty ($D$), Programming Effort ($E$), and Estimated Bugs ($B = \frac{V}{3000}$).
- **Maintainability Index (MI)**:
  $$\text{MI} = 171 - 5.2 \ln(V) - 0.23 M - 16.2 \ln(\text{LOC})$$
- **Tree-sitter AST Parsing**: Multi-language syntax tree validation with modular feature flags (**Rust**, **TypeScript/JavaScript**, **Python**, **Go**, **Java**, **C**, **C++**).
- **Diff & Regression Risk (LCS)**: Analyzes before/after code deltas using Longest Common Subsequence, computing complexity deltas ($\Delta M$, $\Delta\text{MI}$), function blast radius, and composite regression risk.
- **Boundary Analysis**: Detects unhandled `.unwrap()`, unsafe memory access, bare exceptions, ignored errors, buffer overflow risks, and unchecked array indices (`[0]`).

### 5. Set Theory & Dynamic Contradiction Detection
- **Semantic Constraint Matching**: Uses n-gram Jaccard similarity $J(A, B) = \frac{|A \cap B|}{|A \cup B|}$ and token overlap to match requirements semantically without relying on fragile substring containment.
- **Dynamic Negation Parsing**: Automatically flags logical contradictions ($P \wedge \neg P \models \bot$) by extracting negated concepts and conflicting assertions.

---

## 🛠️ MCP Tools Exposed

| Tool Name | Type | Description |
|---|---|---|
| `math_audit_cognition` | **Unified Gate** | Complete metacognition audit (Requirements + DAG + Text Entropy + Confidence + Code Metrics) with weighted scoring. Returns `PASS`/`FAIL` verdict and remediation steps. |
| `math_track_dag` | Granular | Initializes and tracks plan execution against a formal DAG. Detects cycles, dependency violations, and unapproved tasks. |
| `math_eval_code` | Granular | Computes AST, McCabe Cyclomatic Complexity, Halstead measures, Maintainability Index, and boundary warnings across supported languages. |
| `math_eval_diff` | Granular | Analyzes code diffs via LCS, calculating line change ratio, complexity deltas ($\Delta M$, $\Delta\text{MI}$), touched functions, and regression risk. |
| `math_eval_text` | Granular | Analyzes Shannon entropy, information density, compression ratio, Flesch readability, AI filler counts, and optimal length. |
| `math_confidence` | Granular | Evaluates AI responses for confidence score, hedging ratio, assertion density, concrete specificity, and self-contradictions. |
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
