---
name: math-metacognition
description: Formal mathematical verification and metacognition protocol. Uses discrete math, DAG execution graphs, Shannon entropy, Halstead/McCabe complexity, epistemic calibration, research gating, and foresight analysis to eliminate AI hallucinations, scope creep, overconfidence, and verbosity.
when_to_use: "Always use when planning, implementing, coding, or before presenting final solutions to verify logical soundness, code quality, grounding, and calibration."
allowed-tools: Read, Glob, Grep, call_mcp_tool
version: 1.3.0
---

# Math Metacognition Skill

> **Goal:** Transform AI reasoning from stochastic hallucinations into deterministic, mathematically proven outcomes with true epistemic calibration.

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
- Contradiction check: Ensure no $P \wedge \neg P$ conditions exist using dynamic negation detection.

### C. Information Theory (Shannon Entropy & Text Density)
- **Shannon Entropy:** $H(X) = -\sum p(x) \log_2 p(x)$
- **Information Density:** $D = H(X) \times \text{TTR}$
- **Compression Ratio (Kolmogorov proxy):** Filters low-entropy repetitive fluff.
- **AI Filler Penalty:** Drops scores for conversational padding ("As an AI...", "I'd be happy to...").

### D. Epistemic Calibration & Anti-Overconfidence
- **Calibration Index:** Prevents blind overconfidence. Unverified absolute assertions ("guaranteed", "100%", "chắc chắn") are heavily penalized.
- **Verdict Mapping:**
  - `CALIBRATED`: Grounded in empirical facts, confident without arrogance.
  - `OVERCONFIDENT`: Absolute assertions without proof.
  - `UNDERCONFIDENT`: High technical content but needlessly hesitant.
  - `EVASIVE`: Empty conversational dodging.

### E. Research Gate & Empirical Grounding
- **Evidence Ratio:** $E = \frac{\text{citations}}{\max(\text{factual\_claims}, 1)}$
- **Research Deficit Detection:** If factual claims are made with 0 citations (URLs, RFCs, paths, logs), flags `RESEARCH_DEFICIT`.

### F. Proactive Foresight & Software Complexity
- **Defensive Design:** Explicit error handling, timeouts, fallbacks.
- **Boundary & Edge Case Coverage:** Limits, zero/empty states, concurrency.
- **McCabe Cyclomatic Complexity:** $M \le 10$ per function.
- **Maintainability Index:** $MI \ge 65$.
- **LCS Diff Regression Risk:** Analyzes complexity deltas before committing changes.

---

## 2. Using the Rust MCP Plugin

### Step 1: Pre-execution Constraint, DAG & Foresight Verification
- Invoke `math_verify_constraints` to check requirement consistency.
- Invoke `math_track_dag` to validate plan topological ordering.
- Invoke `math_eval_foresight` to ensure plan depth is sufficient.

### Step 2: Research & Code Diff Checks
- Call `math_audit_research` when asserting technical capabilities.
- Call `math_eval_diff` when proposing code modifications.

### Step 3: Unified Metacognition Gate
Call `math_audit_cognition` with:
- `user_requirements`
- `planned_tasks`
- `executed_steps`
- `draft_response`
- `code_snippet`
- `language`

Evaluate the returned `verdict`. If `FAIL`, adjust output according to `recommendations` and re-audit until `PASS`.
