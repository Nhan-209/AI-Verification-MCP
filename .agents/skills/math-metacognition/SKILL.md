---
name: math-metacognition
description: Formal mathematical verification and metacognition protocol. Uses discrete math, DAG execution graphs, Shannon entropy, Halstead/McCabe complexity, and AST parsing to eliminate AI hallucinations, scope creep, and verbosity.
when_to_use: "Always use when planning, implementing, coding, or before presenting final solutions to verify logical soundness, code quality, and alignment."
allowed-tools: Read, Glob, Grep, call_mcp_tool
version: 1.0.0
---

# Math Metacognition Skill

> **Goal:** Transform AI reasoning from stochastic hallucinations into deterministic, mathematically proven outcomes.

---

## 1. Mathematical Pillars

### A. Graph Theory Plan Tracking (DAG)
- **Model:** Directed Acyclic Graph $G = (V, E)$.
- **Coverage Ratio:**
  $$C = \frac{|V_{\text{exec}} \cap V_{\text{plan}}|}{|V_{\text{plan}}|}$$
  Must reach $1.0$ ($100\%$) upon completion.
- **Waste / Scope Creep:**
  $$W = |V_{\text{exec}} \setminus V_{\text{plan}}|$$
  Must strictly equal $0$. Any unapproved task is waste.

### B. Set Theory Requirement Alignment
- Missing requirements: $\Delta_{\text{missing}} = R_{\text{user}} \setminus R_{\text{impl}} = \emptyset$.
- Contradiction check: Ensure no $P \wedge \neg P$ conditions exist in interpretation.

### C. Information Theory (Shannon Entropy & Text Density)
- **Shannon Entropy:** $H(X) = -\sum p(x) \log_2 p(x)$
- **Information Density:** $D = H(X) \times \text{TTR}$
- **Compression Ratio (Kolmogorov proxy):** Avoid low compression ratios on high word counts (indicates repetitive boilerplate).
- **Readability:** Flesch Reading Ease score $> 40$, Gunning Fog Index $< 16$.

### D. Code Quality & Software Complexity
- **McCabe Cyclomatic Complexity:** $M = E - N + 2P \le 10$ per function.
- **Maintainability Index:**
  $$MI = 171 - 5.2 \ln(V) - 0.23 M - 16.2 \ln(\text{LOC})$$
  Target: $MI \ge 65$ (Production Grade).
- **Boundary Conditions:** Verify safety against `unwrap()`, unhandled `None`, unchecked array indices `[0]`, bare `except:`, and `any` types.

---

## 2. Using the Rust MCP Plugin

### Step 1: Pre-execution Constraint & DAG Verification
Invoke `math_track_dag` or `math_verify_constraints` to ensure requirements are well-formed.

### Step 2: Continuous Execution Check
Record each executed task in the DAG. Ensure no dependency violations occur.

### Step 3: Unified Metacognition Gate
Call `math_audit_cognition` with:
- `user_requirements`
- `planned_tasks`
- `executed_steps`
- `draft_response`
- `code_snippet`
- `language`

Evaluate the returned `verdict`. If `FAIL`, adjust output according to `recommendations`.
