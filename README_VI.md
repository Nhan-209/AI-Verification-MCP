# 🛡️ MCP Plugin Math: Tầng Quản Trị & Kiểm Toán Nhận Thức Cho AI Agent

[English](README.md) | [Tiếng Việt](README_VI.md)

[![Rust CI/CD](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml/badge.svg)](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Protocol: MCP 2024-11-05](https://img.shields.io/badge/MCP-2024--11--05-brightgreen.svg)](https://modelcontextprotocol.io)
[![Version: 0.5.0](https://img.shields.io/badge/version-0.5.0-orange.svg)](Cargo.toml)

Một **Model Context Protocol (MCP)** Server hiệu năng cực cao viết bằng **Rust**, đóng vai trò là **Tầng Quản Trị & Kiểm Toán (Verification & Governance Layer)** cho các hệ thống AI Agent. Dự án chuyển hóa các chỉ số toán học và phân tích tĩnh thành các rào chắn kỹ thuật (guardrails), áp dụng cơ chế phán quyết 3 cấp (**`ALLOW`**, **`WARN`**, **`BLOCK`**) với mã vi phạm chuẩn hóa và kế hoạch khắc phục hành động cụ thể (actionable remediation).

---

## 🌟 Triết Lý: Rào Chắn Kỹ Thuật Tất Định Thay Vì Phỏng Đoán

Các Mô hình Ngôn ngữ Lớn (LLM) bản chất là các bộ dự đoán token theo xác suất. Trong các luồng tương tác đa tác tử (multi-agent workflows), việc thiếu sự kiểm soát tất định dẫn tới:
1. **Ảo giác & Lệch mục tiêu (Hallucination & Drift)**: Bịa đặt API hoặc tự ý đi xa khỏi yêu cầu người dùng.
2. **Làm thừa việc không kiểm soát ($W > 0$)**: Tự ý code hoặc thực thi tác vụ ngoài kế hoạch gây phá vỡ kiến trúc.
3. **Dài dòng & Hao tổn Token**: Mật độ thông tin nghèo nàn, nhiều từ đệm khách sáo, nội dung lặp lại.
4. **Tự tin thái quá (Overconfidence)**: Tuyên bố "chắc chắn 100%", "hoàn hảo không lỗi" khi không hề có dữ liệu thực nghiệm.
5. **Lười nghiên cứu (Research Deficit)**: Đoán mò tính năng thư viện, thông số API thay vì viện dẫn RFC và tài liệu.
6. **Lập kế hoạch hời hợt (Lazy Plan)**: Lập kế hoạch 1 bước sơ sài cho bài toán phức tạp, bỏ quên ca biên và xử lý lỗi.

`mcp-plugin-math` cung cấp một **Cổng Quản Trị & Kiểm Toán (Governance Gate)** độc lập. Trước khi thực thi hành động quan trọng hoặc gửi câu trả lời hoàn tất, AI gửi kế hoạch và bản nháp tới cổng kiểm toán để nhận phản hồi cấu trúc:

```
[Đề Xuất Của AI] ──► [Cổng Kiểm Toán mcp-plugin-math] ──► ALLOW | WARN | BLOCK
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
- **Tỷ lệ Do dự & Quyết đoán**: Phân biệt sự thận trọng khoa học với việc nói né tránh, vòng vo.
- **Phát hiện Tự Mâu Thuẫn**: Bắt lỗi mâu thuẫn logic trong cùng một nội dung phản hồi ($P \wedge \neg P$).

### 3. Cổng Nghiên Cứu Thực Nghiệm (Research Gate)
- **Chẩn Đoán Khẳng Định Kỹ Thuật**: Phân loại khẳng định theo nhóm (hiệu năng benchmark, phiên bản đặc tả, tính tương thích API).
- **Xác Thực Dẫn Chứng**: Kiểm tra sự hiện diện của bằng chứng (URL tài liệu, mã RFC, đường dẫn file, log thử nghiệm).
- **Cảnh Báo Thiếu Hụt Nghiên Cứu (`RESEARCH_DEFICIT`)**: Buộc AI phải trích dẫn tài liệu trước khi đưa ra kết luận.

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

### 6. Khớp Ngữ Nghĩa Ràng Buộc & Mâu Thuẫn Phủ Định
- **Đối Soát Ngữ Nghĩa**: Đánh giá mức độ hoàn thành yêu cầu qua tương đồng ký tự n-gram Jaccard $J(A, B) = \frac{|A \cap B|}{|A \cup B|}$.
- **Phân Tích Cú Pháp Phủ Định**: Phát hiện mâu thuẫn giữa cam kết thực thi và ràng buộc người dùng đặt ra.

---

## 🚦 Cơ Chế Phán Quyết 3 Cấp: ALLOW, WARN, BLOCK

Cổng kiểm toán tổng hợp (`math_audit_cognition`) đưa ra quyết định quản trị rõ ràng:

| Quyết Định | Điều Kiện | Hành Vi Của Agent |
|:---:|---|---|
| **`ALLOW`** | Không có vi phạm nghiêm trọng (Critical), điểm tổng hợp $\ge 75\%$. | Được phép tiến hành / bàn giao kết quả. |
| **`WARN`** | Không có vi phạm nghiêm trọng, nhưng có cảnh báo (ví dụ câu hơi dài, làm việc ngoài kế hoạch nhẹ, hoặc điểm $50-75\%$). | Được phép tiến hành nhưng nên lưu ý khuyến nghị. |
| **`BLOCK`** | Có vi phạm nghiêm trọng (lỗi cú pháp AST, sai thứ tự DAG, thiếu nghiên cứu, tự tin thái quá, thiếu yêu cầu) hoặc điểm $< 50\%$. | Dừng thực thi ngay lập tức; AI bắt buộc phải khắc phục theo `remediation_plan` trước khi tiếp tục. |

### Cấu Trúc Vi Phạm Chuẩn Hóa

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

## ⚡ Các Chế Độ Thực Thi (`mode`)

- **`quick`**: Kiểm toán siêu tốc (<1ms). Chỉ chạy kiểm tra ràng buộc cốt lõi và hiệu chuẩn nhận thức; bỏ qua việc duyệt cây AST và văn bản tiên liệu. Phù hợp cho việc tự kiểm tra liên tục giữa các bước lặp.
- **`standard`** (mặc định): Kiểm toán toàn diện 6 trụ cột (<5ms). Phù hợp cho việc chuyển giao các giai đoạn tác vụ.
- **`deep`**: Kiểm toán tối đa với tiêu chuẩn khắt khe hơn (ngưỡng McCabe $\le 15$, MI $\ge 65$, bắt buộc DAG phải hoàn thành 100%). Phù hợp cho khâu kiểm tra cuối trước khi tạo PR hoặc release.

---

## 🛠️ Danh Sách 9 MCP Tools

| Tool Name | Loại | Mô Tả |
|---|---|---|
| **`math_audit_cognition`** | **Cổng Tổng Hợp** | Kiểm toán toàn diện 6 trụ cột. Trả về phán quyết 3 cấp (`ALLOW`/`WARN`/`BLOCK`), danh sách vi phạm và kế hoạch khắc phục. Hỗ trợ 3 chế độ `quick`, `standard`, `deep`. |
| **`math_track_dag`** | Chi Tiết | Theo dõi tiến trình DAG. Bắt lỗi chu trình, sai thứ tự tô pô và hỗ trợ phân loại khám phá hợp lệ. |
| **`math_eval_code`** | Chi Tiết | Phân tích AST qua Tree-sitter, McCabe Cyclomatic, Halstead, Maintainability Index và cảnh báo biên cho 7 ngôn ngữ. |
| **`math_eval_diff`** | Chi Tiết | Phân tích diff bằng LCS, tính toán tỷ lệ thay đổi, biến thiên độ phức tạp ($\Delta M$, $\Delta\text{MI}$), hàm bị ảnh hưởng và rủi ro hồi quy. |
| **`math_eval_text`** | Chi Tiết | Đo lường Shannon entropy, mật độ thông tin, tỷ lệ nén, chỉ số khả năng đọc và phát hiện từ đệm AI. |
| **`math_confidence`** | Chi Tiết | Hiệu chuẩn nhận thức. Bắt lỗi tự tin thái quá đồng thời công nhận các khẳng định chỉ số thực nghiệm và hợp đồng đặc tả. |
| **`math_audit_research`** | Chi Tiết | Rà soát khẳng định kỹ thuật (benchmark, phiên bản, API), kiểm tra dẫn chứng (RFC, URL, log) và cảnh báo thiếu hụt nghiên cứu. |
| **`math_eval_foresight`** | Chi Tiết | Đánh giá thiết kế phòng thủ, ca biên, chiến lược kiểm thử và cảnh báo kế hoạch sơ sài. |
| **`math_verify_constraints`** | Chi Tiết | Đối soát yêu cầu với giải pháp thực tế bằng n-gram Jaccard và phát hiện mâu thuẫn phủ định động. |

---

## 📦 Hỗ Trợ Ngôn Ngữ & Feature Flags

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

File binary độc lập cho **Linux (x86_64)** và **Windows (x86_64)** được tự động biên dịch trên mỗi commit/release qua [GitHub Actions](https://github.com/Nhan-209/mcp-plugin-math/actions).

### Cấu Hình MCP Clients

Thêm vào tệp cấu hình của Claude Desktop, Antigravity, hoặc Gemini CLI:

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

Hoặc trên Windows:
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

## 📜 Giấy Phép
Phát hành theo [Giấy phép MIT](LICENSE).
