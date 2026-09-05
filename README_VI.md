# 🛡️ AI Verification MCP: Tầng Thực Thi Chính Sách & Bằng Chứng Tất Định Cho AI Agent

[English](README.md) | [Tiếng Việt](README_VI.md)

[![Rust CI/CD](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml/badge.svg)](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Protocol: MCP 2024-11-05](https://img.shields.io/badge/MCP-2024--11--05-brightgreen.svg)](https://modelcontextprotocol.io)
[![Version: 0.9.0](https://img.shields.io/badge/version-0.9.0-orange.svg)](Cargo.toml)

Một **Model Context Protocol (MCP)** Server hiệu năng cực cao viết bằng **Rust**, đóng vai trò là **Tầng Thực Thi Chính Sách & Bằng Chứng Tất Định (Deterministic Evidence & Policy Enforcement Layer)** cho các hệ thống AI Agent. Dự án chuyển hóa các tín hiệu tất định, phân tích cú pháp AST và danh mục nguồn dẫn uy tín thành các rào chắn kỹ thuật (guardrails), áp dụng cơ chế phán quyết 4 cấp (**`ALLOW`**, **`WARN`**, **`BLOCK`**, **`INSUFFICIENT_EVIDENCE`**) với mã vi phạm chuẩn hóa và kế hoạch khắc phục hành động cụ thể (actionable remediation).

---

## ⚖️ Mô Hình Tin Cậy & Biên Giới Cam Kết (Trust Model & Epistemic Boundaries)

`ai-verification-mcp` vận hành dưới một mô hình tin cậy toán học rõ ràng và có biên giới giới hạn:
$$\text{ALLOW} \equiv \text{Không phát hiện vi phạm chính sách trong phạm vi bằng chứng đã khai báo}$$
$$\text{ALLOW} \not\equiv \text{Bằng chứng thực nghiệm rằng AI là toàn tri hay không thể mắc lỗi}$$

Hệ thống loại bỏ hoàn toàn sự phụ thuộc vào các mô hình "LLM-as-a-judge" cảm tính bằng cách áp đặt **các bất biến tất định (hard deterministic invariants)**:
1. **Tiên đề Dẫn chứng Toàn diện (Universal Grounding Axiom)**: Nếu AI đưa ra các luận điểm kỹ thuật cụ thể, từng luận điểm đều phải có nguồn trích dẫn được xác thực. Dẫn chứng hợp lệ cho Luận điểm A không thể dùng để bảo kê cho Luận điểm B không có bằng chứng.
2. **Ma trận Bằng chứng Bắt buộc (Mandatory Evidence Matrix)**: Payload khuyết thiếu không thể gian lận điểm số. Ở chế độ `standard`, việc thiếu hợp đồng (`user_requirements` hoặc `planned_tasks`) bắt buộc trả về `INSUFFICIENT_EVIDENCE`. Ở chế độ `deep`, việc thiếu bất kỳ trụ cột nào sẽ dẫn đến `BLOCK`.
3. **Danh Mục Tiêu Chuẩn Chuẩn Hóa (`KNOWN_RFC_REGISTRY`)**: Loại bỏ việc chấp nhận dải số ngẫu nhiên. Số hiệu RFC phải đối chiếu với `KNOWN_RFC_REGISTRY` và các tổ chức tiêu chuẩn phải đi kèm mã định danh cấu trúc (`IEEE 754`, `ISO/IEC 27001`).

---

## 🌟 Triết Lý: Rào Chắn Tín Hiệu Tất Định Kiểm Soát & Giám Sát Tác Tử AI

Các Mô hình Ngôn ngữ Lớn (LLM) bản chất là các bộ dự đoán token theo xác suất. Dù LLM không thể chứng minh toán học rằng mình không có ảo giác ngữ nghĩa, một tầng kiểm toán tất định độc lập **hoàn toàn có thể** kiểm soát và ràng buộc các sản phẩm suy luận của agent (kế hoạch, mã diff, dẫn chứng thực nghiệm, mức độ tự tin):

1. **Ảo giác & Lệch mục tiêu (Hallucination & Drift)**: Bịa đặt API không căn cứ hoặc tự ý đi xa khỏi yêu cầu người dùng.
2. **Làm thừa việc không kiểm soát ($W > 0$)**: Tự ý code hoặc thực thi tác vụ ngoài kế hoạch gây phá vỡ kiến trúc.
3. **Dài dòng & Hao tổn Token**: Mật độ thông tin nghèo nàn, nhiều từ đệm khách sáo, lãng phí token vô ích.
4. **Tự tin thái quá (Overconfidence)**: Tuyên bố "chắc chắn 100%", "hoàn hảo không lỗi" khi không hề có dữ liệu thực nghiệm.
5. **Lười nghiên cứu (Research Deficit)**: Đoán mò tính năng thư viện, thông số API thay vì viện dẫn RFC và tài liệu.
6. **Lập kế hoạch hời hợt (Lazy Plan)**: Lập kế hoạch 1 bước sơ sài cho bài toán phức tạp, bỏ quên ca biên và xử lý lỗi.

`ai-verification-mcp` cung cấp một **Cổng Quản Trị & Kiểm Toán (Governance Gate)** độc lập. Trước khi thực thi hành động quan trọng hoặc gửi câu trả lời hoàn tất, AI gửi kế hoạch, mã diff và bản nháp tới cổng kiểm toán để nhận phản hồi cấu trúc:

```
[Đề Xuất Của AI] ──► [Cổng Kiểm Toán ai-verification-mcp] ──► ALLOW | WARN | BLOCK | INSUFFICIENT_EVIDENCE
                                                                   ▲
                                                        Mã Vi Phạm & Kế Hoạch
                                                        Khắc Phục Cụ Thể
```

---

## 🔬 Các Trụ Cột Kiểm Toán

### 1. Kiểm Toán Kế Hoạch Qua DAG & Khám Phá Hợp Lệ (Justified Discovery)
Kế hoạch thực thi được mô hình hóa thành Đồ thị Có hướng Không Chu trình (DAG) $G = (V, E)$.
- **Tỷ lệ bao phủ kế hoạch (Coverage Ratio)**:
  $$C = \frac{|V_{\text{thực\_thi}} \cap V_{\text{kế\_hoạch}}|}{|V_{\text{kế\_hoạch}}|}$$
- **Phân biệt Khám Phá Hợp Lệ với Scope Creep**:
  Hệ thống nhận diện các tác vụ mang tính điều tra/khám phá thông tin (`view_file`, `cargo_test`, `grep_search`, `inspect_logs`, v.v.) là **Khám Phá Hợp Lệ (JustifiedDiscovery)**, không phạt điểm làm thừa như các hành động tùy tiện.
- **Bất biến Tô pô**: Đảm bảo các tác vụ tiên quyết luôn phải hoàn thành trước tác vụ phụ thuộc.

### 2. Hiệu Chuẩn Nhận Thức Phù Hợp Ngữ Cảnh (Context-Aware Confidence)
- **Nhận diện Ngữ Cảnh Đo Lường Kỹ Thuật**: Phân biệt rõ ràng giữa các khẳng định chỉ số kỹ thuật thực nghiệm (ví dụ: `"100% test coverage"`, `"p99 latency < 2ms"`) hay hợp đồng đặc tả (`"guaranteed by RFC 2119"`) với sự tự tin thái quá vô căn cứ.
- **Danh Sách Phủ Định Khiêm Tốn (Cautious Negation)**: Các câu từ chối cam kết khiêm tốn khoa học (ví dụ: `"không đảm bảo"`, `"does not prove"`) được bảo vệ, không bị phạt là overconfidence.
- **Chống Rửa Dẫn Chứng (Evidence Laundering)**: Ràng buộc dẫn chứng theo từng câu đơn lẻ thay vì cho phép 1 link ở đầu bảo kê toàn bộ các tuyên bố vô căn cứ bên dưới.
- **Phát hiện Tự Mâu Thuẫn**: Bắt lỗi mâu thuẫn logic trong cùng một nội dung phản hồi ($P \wedge \neg P$).

### 3. Cổng Nghiên Cứu Thực Nghiệm & Vòng Đời Dẫn Chứng 3 Cấp
- **Quy Trình Xác Thực Dẫn Chứng 3 Cấp**:
  $$\text{Unsupported} \longrightarrow \text{EvidencePresent} \longrightarrow \text{EvidenceVerified}$$
  Kiểm tra số hiệu RFC đối chiếu với `KNOWN_RFC_REGISTRY`, domain uy tín (`docs.rs`, `ietf.org`, `crates.io`, `github.com`), đường dẫn file nội bộ, và tiêu chuẩn có cấu trúc (`IEEE 754`, `ISO/IEC 27001`); từ chối domain giữ chỗ (`example.com`) và RFC chưa phân loại.
- **Chẩn Đoán Khẳng Định Kỹ Thuật**: Phân loại khẳng định theo nhóm (hiệu năng benchmark, phiên bản đặc tả, tính tương thích API).
- **Thực Thi Tiên Đề Dẫn Chứng Toàn Diện**: Bất kỳ khẳng định nào thiếu dẫn chứng đều kích hoạt `RESEARCH_DEFICIT` (Critical) và chặn bàn giao.

### 4. Lý Thuyết Thông Tin & Tách Câu Thông Minh
- **Shannon Entropy**:
  $$H(X) = -\sum_{x} p(x) \log_2 p(x)$$
- **Mật độ Thông Tin**: $D = H(X) \times \text{TTR}$ giúp loại bỏ câu đệm sáo rỗng.
- **Bộ Tách Câu Không Phá Vỡ (`text_utils.rs`)**: Bảo vệ nguyên vẹn các định dạng kỹ thuật: URL (`github.com/...`), phiên bản (`v1.2.3`), số thập phân (`3.14`), chữ viết tắt (`e.g.`, `vd.`).
- **Từ Điển Câu Đệm Song Ngữ EN/VI**: Nhận diện câu đệm khách sáo trong cả tiếng Anh và tiếng Việt.

### 5. Tư Duy Tiên Liệu & Độ Phức Tạp Phần Mềm
- **Kiểm Tra Thiết Kế Phòng Thủ**: Đánh giá xử lý ngoại lệ, timeout, cơ chế thử lại (retry) và phương án dự phòng (fallback).
- **Bao Phủ Trường Hợp Biên**: Kiểm tra trạng thái rỗng, giới hạn biên, tranh chấp tài nguyên.
- **Chống Kế Hoạch Sơ Sài (`LAZY_PLAN`)**: Phát hiện việc chia nhỏ nhiệm vụ không tương xứng với yêu cầu bài toán.
- **McCabe Cyclomatic Complexity**: Giới hạn độ phức tạp mã nguồn ($M \le 15-20$).
- **Chỉ Số Bảo Trì (MI)**: Đảm bảo khả năng bảo trì đạt chuẩn production.
- **Phân Tích Khác Biệt Mã Nguồn (LCS)**: Đánh giá rủi ro hồi quy trước khi commit.

### 6. Khớp Ngữ Nghĩa Ràng Buộc & Chống Tráo Đổi Thực Thể
- **Đối Soát Ngữ Nghĩa**: Đánh giá mức độ hoàn thành yêu cầu qua tương đồng ký tự n-gram Jaccard $J(A, B) = \frac{|A \cap B|}{|A \cup B|}$.
- **Chống Tráo Đổi Thực Thể (Entity Substitution Guard)**: Ngăn chặn việc hoán đổi danh từ thực thể cốt lõi của bài toán (ví dụ: thay `"secrets"` bằng `"logs"`).
- **Phân Tích Cú Pháp Phủ Định**: Phát hiện mâu thuẫn giữa cam kết thực thi và ràng buộc người dùng đặt ra.

---

## 🚦 Cơ Chế Phán Quyết 4 Cấp: ALLOW, WARN, BLOCK, INSUFFICIENT_EVIDENCE

Cổng kiểm toán tổng hợp (`verify_agent`) đưa ra quyết định quản trị rõ ràng:

| Quyết Định | Điều Kiện | Hành Vi Của Agent |
|:---:|---|---|
| **`ALLOW`** | Đầy đủ bằng chứng bắt buộc, 0 vi phạm Critical, 0 vi phạm Warning, điểm chính sách $\ge 75\%$. | Được phép tiến hành / bàn giao kết quả. |
| **`WARN`** | Đầy đủ bằng chứng bắt buộc, không có vi phạm Critical, nhưng có cảnh báo Warning (hoặc điểm $50-75\%$). | Được phép tiến hành nhưng nên lưu ý khuyến nghị. |
| **`BLOCK`** | Có bất kỳ vi phạm nghiêm trọng nào (lỗi cú pháp, sai thứ tự DAG, luận điểm thiếu nguồn, tự tin thái quá, thiếu yêu cầu) hoặc điểm $< 50\%$. | Dừng thực thi ngay lập tức; AI bắt buộc phải khắc phục theo `remediation_plan` trước khi tiếp tục. |
| **`INSUFFICIENT_EVIDENCE`** | Payload trống rỗng hoặc thiếu hợp đồng bắt buộc (ví dụ: chỉ gửi văn bản nháp ở chế độ `standard` mà không có yêu cầu hay kế hoạch). | Chưa đủ bằng chứng để kiểm toán; AI cần cung cấp hợp đồng yêu cầu hoặc kế hoạch cụ thể. |

### Cấu Trúc Vi Phạm Chuẩn Hóa

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

## ⚡ Các Chế Độ Thực Thi (`mode`)

- **`quick`**: Kiểm toán siêu tốc (<1ms). Chỉ chạy kiểm tra ràng buộc cốt lõi và hiệu chuẩn nhận thức; bỏ qua việc duyệt cây AST và văn bản tiên liệu. Phù hợp cho việc tự kiểm tra liên tục giữa các bước lặp.
- **`standard`** (mặc định): Kiểm toán toàn diện 6 trụ cột (<5ms). Phù hợp cho việc chuyển giao các giai đoạn tác vụ.
- **`deep`**: Kiểm toán tối đa với tiêu chuẩn khắt khe hơn (ngưỡng McCabe $\le 15$, MI $\ge 65$, bắt buộc DAG phải hoàn thành 100%). Phù hợp cho khâu kiểm tra cuối trước khi tạo PR hoặc release.

---

## 🛠️ Danh Sách MCP Tools (Tên Mới & Bí Danh Tương Thích)

Toàn bộ công cụ hỗ trợ 100% bí danh ngược (backward-compatible) cho các client đang dùng tên `math_*`.

| Tên Công Cụ Chính | Bí Danh Kế Thừa | Loại | Mô Tả |
|---|---|---|---|
| **`verify_agent`** | `math_audit_cognition`, `ai_audit_cognition` | **Cổng Tổng Hợp** | Kiểm toán toàn diện 6 trụ cột. Trả về **phán quyết 4 cấp** (`ALLOW`/`WARN`/`BLOCK`/`INSUFFICIENT_EVIDENCE`), danh sách vi phạm và kế hoạch khắc phục. Hỗ trợ 3 chế độ `quick`, `standard`, `deep` và tham số `audit_phase` (`plan`/`execution`). |
| **`verify_dag`** | `math_track_dag` | Chẩn Đoán | Theo dõi tiến trình DAG. Bắt lỗi chu trình, tham chiếu dependency không tồn tại, trùng lặp mã task và phân loại khám phá hợp lệ. |
| **`verify_code`** | `math_eval_code` | Chẩn Đoán | Phân tích AST qua Tree-sitter, McCabe Cyclomatic, Halstead xấp xỉ, Maintainability Index và cảnh báo biên cho 7 ngôn ngữ. |
| **`verify_diff`** | `math_eval_diff` | Chẩn Đoán | Phân tích diff bằng LCS, tính toán tỷ lệ thay đổi, biến thiên độ phức tạp ($\Delta M$, $\Delta\text{MI}$), hàm bị ảnh hưởng và rủi ro hồi quy. |
| **`verify_text`** | `math_eval_text` | Chẩn Đoán | Đo lường Shannon entropy, mật độ thông tin, tỷ lệ nén, chỉ số khả năng đọc và phát hiện từ đệm AI. |
| **`verify_confidence`** | `math_confidence` | Chẩn Đoán | Hiệu chuẩn nhận thức. Bắt lỗi tự tin thái quá đồng thời công nhận các khẳng định chỉ số thực nghiệm và hợp đồng đặc tả. |
| **`verify_research`** | `math_audit_research` | Chẩn Đoán | Rà soát khẳng định kỹ thuật (benchmark, phiên bản, API), kiểm tra RFC đối chiếu với danh mục chuẩn hóa 70+ RFC thực tế và tên miền xác thực. |
| **`verify_foresight`** | `math_eval_foresight` | Chẩn Đoán | Đánh giá văn phong dự liệu phòng thủ, ca biên, chiến lược kiểm thử và cảnh báo kế hoạch sơ sài. |
| **`verify_constraints`** | `math_verify_constraints` | Chẩn Đoán | Đối soát yêu cầu với giải pháp thực tế bằng n-gram Jaccard và phát hiện mâu thuẫn phủ định động. |

---

## 📦 Hỗ Trợ Ngôn Ngữ & Feature Flags

`ai-verification-mcp` sử dụng Cargo feature flags để biên dịch theo module:

| Feature Flag | Ngôn Ngữ Hỗ Trợ | Mặc Định? |
|---|---|:---:|
| `lang-rust` | Rust | ✅ |
| `lang-typescript` | TypeScript, JavaScript, TSX | ✅ |
| `lang-python` | Python | ✅ |
| `lang-go` | Go | Tùy chọn |
| `lang-java` | Java | Tùy chọn |
| `lang-c` | C | Tùy chọn |
| `lang-cpp` | C++ | Tùy chọn |
| `all-languages` | Tất cả 7 ngôn ngữ | Tùy chọn |

---

## 🚀 Cài Đặt & Tích Hợp

File binary độc lập cho **Linux (x86_64)** và **Windows (x86_64)** được tự động biên dịch trên mỗi commit/release qua [GitHub Actions](https://github.com/Nhan-209/mcp-plugin-math/actions). Cả hai file thực thi `ai-verification-mcp` và `mcp-plugin-math` đều được xuất xưởng giống hệt nhau.

### Cấu Hình MCP Clients

Thêm vào tệp cấu hình của Claude Desktop, Antigravity, hoặc Gemini CLI:

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

Hoặc trên Windows:
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

## 📜 Giấy Phép
Phát hành theo [Giấy phép MIT](LICENSE).
