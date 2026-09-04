# 🧮 MCP Plugin Math: Formal Metacognition & Anti-Hallucination Engine for AI

[![Rust CI/CD](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml/badge.svg)](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Protocol: MCP 2024-11-05](https://img.shields.io/badge/MCP-2024--11--05-brightgreen.svg)](https://modelcontextprotocol.io)

An ultra-high-performance **Model Context Protocol (MCP)** Server written in **Rust** that empowers AI Agents to eliminate hallucinations, prevent scope creep, control verbosity, and ensure production-grade code through **deterministic mathematical proofs and formal software metrics**.

---

## 🌟 The Philosophy: From Stochastic Guessing to Mathematical Proofs

Large Language Models (LLMs) are inherently probabilistic token predictors. Without grounding, they suffer from:
1. **Hallucination & Drift**: Fabricating facts or deviating from initial intentions.
2. **Scope Creep ($W > 0$)**: Implementing unasked, redundant, or disruptive code.
3. **Fluff & Verbosity**: Wasting tokens with repetitive, dry, or unnecessarily complex prose.
4. **Sub-optimal Code**: Hidden logic flaws, unhandled boundary cases, and excessive cyclomatic complexity.

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
- **Readability Indices**: Flesch Reading Ease & Gunning Fog Index ensure outputs remain clear, engaging, and neither dry nor convoluted.

### 3. Software Complexity & AST Analysis
- **McCabe Cyclomatic Complexity**: $M = E - N + 2P$. Flags functions with $M > 10$.
- **Halstead Metrics**: Computes Program Volume ($V$), Difficulty ($D$), Programming Effort ($E$), and Estimated Bugs ($B = \frac{V}{3000}$).
- **Maintainability Index (MI)**:
  $$\text{MI} = 171 - 5.2 \ln(V) - 0.23 M - 16.2 \ln(\text{LOC})$$
- **Tree-sitter AST Parsing**: Native multi-language syntax tree validation for **Rust**, **TypeScript/JavaScript**, and **Python**. Identifies syntax error nodes directly in the AST.
- **Boundary Analysis**: Detects unhandled `.unwrap()`, unsafe memory access, bare exceptions, and unchecked array indices (`[0]`).

### 4. Set Theory & Contradiction Detection
- **Missing Requirements**: $\Delta_{\text{missing}} = R_{\text{req}} \setminus R_{\text{impl}}$.
- **Formal Non-Contradiction**: Detects mutually exclusive directives ($P \wedge \neg P \models \bot$).

---

## 🛠️ MCP Tools Exposed

| Tool Name | Type | Description |
|---|---|---|
| `math_audit_cognition` | **Unified Gate** | Complete metacognition audit (Requirements + DAG + Text Entropy + Code Metrics) in a single call. Returns `PASS`/`FAIL` verdict and remediation steps. |
| `math_track_dag` | Granular | Initializes and tracks plan execution against a formal DAG. Detects cycles, dependency violations, and unapproved tasks. |
| `math_eval_code` | Granular | Computes AST, McCabe Cyclomatic Complexity, Halstead measures, Maintainability Index, and boundary warnings. |
| `math_eval_text` | Granular | Analyzes Shannon entropy, information density, TTR, compression ratio, and Flesch readability. |
| `math_verify_constraints` | Granular | Formal set-theory comparison between user requirements and actual deliverables. |

---

## 🚀 Installation & Integration

### Pre-built Binaries (GitHub Actions)
Pre-compiled standalone binaries for **Linux (x86_64)** and **Windows (x86_64)** are automatically built on every release and commit via [GitHub Actions](https://github.com/Nhan-209/mcp-plugin-math/actions).

### Configuring with MCP Clients

Add to your `claude_desktop_config.json` or Antigravity/Gemini CLI MCP configuration:

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
