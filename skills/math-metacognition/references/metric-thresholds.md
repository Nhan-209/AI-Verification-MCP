# Mathematical Metric Thresholds & Formulas

## 1. Graph & Plan Metrics
| Metric | Formula | Target Threshold | Interpretation |
|---|---|---|---|
| **Coverage ($C$)** | $\frac{\|V_{\text{exec}} \cap V_{\text{plan}}\|}{\|V_{\text{plan}}\|}$ | $1.0$ ($100\%$) | All planned tasks executed |
| **Waste ($W$)** | $\|V_{\text{exec}} \setminus V_{\text{plan}}\|$ | $0$ | Zero unapproved actions / zero scope creep |
| **Topological Drift** | Number of inverted edges in execution sequence | $0$ | Strict dependency satisfaction |

## 2. Information Theory & Text Metrics
| Metric | Target Threshold | Warning Flag | Remediation |
|---|---|---|---|
| **Type-Token Ratio (TTR)** | $\ge 0.45$ | $< 0.35$ | Reduce vocabulary repetition |
| **Compression Ratio (gzip)** | $\ge 0.40$ (on large text) | $< 0.30$ | Cut out filler phrases & boilerplate |
| **Flesch Reading Ease** | $40 - 75$ | $< 30$ (Too dry/convoluted) | Shorten sentences, use clearer words |
| **Gunning Fog Index** | $8 - 14$ | $> 17$ (Academic jargon) | Simplify sentence structures |

## 3. Code Metrics
| Metric | Target Threshold | Warning Flag | Remediation |
|---|---|---|---|
| **McCabe Complexity ($M$)** | $\le 10$ per fn | $> 15$ | Split function, extract sub-routines |
| **Maintainability Index ($MI$)** | $\ge 65$ (Good) | $< 50$ (High risk) | Refactor, reduce LOC and volume |
| **Estimated Bugs ($B$)** | $< 0.05$ per module | $> 0.20$ | Add unit tests, simplify logic |
| **AST Parsing Errors** | $0$ | $> 0$ | Fix syntax errors immediately |
