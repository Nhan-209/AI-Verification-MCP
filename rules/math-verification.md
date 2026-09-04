# Mathematical Verification & Metacognition Protocol (P0 Rule)

> **MANDATORY PROTOCOL:** Formal mathematical verification over probabilistic guessing.
> Before submitting code, complex answers, or task completions, the AI MUST mathematically audit its cognition.

---

## 🎯 Core Mathematical Axioms

1. **Law of Non-Contradiction ($P \wedge \neg P \models \bot$):**
   - AI outputs must NOT contain contradictory logic or breach stated constraints (e.g. "pure rust" vs "node c++ addon").
2. **Principle of Zero Waste ($W = |V_{\text{exec}} \setminus V_{\text{plan}}| = 0$):**
   - Never perform unrequested work or scope creep. Any task not in the approved DAG is flagged as waste ($W > 0$).
3. **Requirement Completeness ($\Delta_{\text{missing}} = R_{\text{req}} \setminus R_{\text{impl}} = \emptyset$):**
   - Every user requirement must be explicitly satisfied.
4. **Information Density Standard ($D \ge 0.40$):**
   - Eliminate filler tokens, repetition, and hollow boilerplate. Maximize Shannon entropy per token.
5. **Software Engineering Formal Metrics:**
   - **McCabe Cyclomatic Complexity:** $M \le 10$ per function ($M \le 20$ project limit).
   - **Maintainability Index:** $MI \ge 65$ (Green Zone).
   - **Zero Parsing Errors:** Code must produce 0 syntax error nodes in AST.

---

## 🛠️ MCP Verification Tool Call Protocol

Whenever the `mcp-plugin-math` MCP server is available:

1. **Before Executing Tasks:**
   - Call `math_verify_constraints` to ensure user requirements have no internal contradictions and are fully mapped.
   - Call `math_track_dag` to validate the topological ordering of the task breakdown.

2. **Before Submitting Final Output / Code:**
   - Call `math_audit_cognition`:
     ```json
     {
       "user_requirements": ["..."],
       "planned_tasks": [{"id": "t1", "name": "...", "dependencies": []}],
       "executed_steps": ["t1", "..."],
       "draft_response": "Draft response text...",
       "code_snippet": "Source code...",
       "language": "rust"
     }
     ```
   - If verdict is `FAIL` or has critical violations, **DO NOT SEND TO USER YET**.
   - Remediate the identified violations first (refactor code, remove scope creep, shorten verbose text), then re-verify until `PASS`.

---
