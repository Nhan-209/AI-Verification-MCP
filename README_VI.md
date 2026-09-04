# 🧮 MCP Plugin Math: Động Cơ Kiểm Chứng Toán Học & Chống Ảo Giác Cho AI

[Tiếng Việt](README_VI.md) | [English](README.md)

[![Rust CI/CD](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml/badge.svg)](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Protocol: MCP 2024-11-05](https://img.shields.io/badge/MCP-2024--11--05-brightgreen.svg)](https://modelcontextprotocol.io)

Một máy chủ **Model Context Protocol (MCP)** hiệu năng siêu cao viết bằng **Rust**, trang bị cho AI Agent khả năng tự kiểm soát nhận thức (metacognition), loại bỏ ảo giác (hallucination), triệt tiêu việc làm thừa (scope creep), kiểm soát độ dài/độ lan man và bảo đảm mã nguồn chuẩn sản xuất thông qua **các chứng minh toán học tất định và hệ thống đo lường phần mềm hình thức**.

---

## 🌟 Triết Lý: Từ Phỏng Đoán Xác Suất Sang Chứng Minh Toán Học

Các mô hình ngôn ngữ lớn (LLM) vốn dĩ là các bộ dự đoán token theo xác suất. Nếu không có cơ chế đối soát tất định, AI sẽ luôn đối mặt với 4 vấn đề kinh điển:
1. **Ảo giác & Lạc đề**: Tự bịa thông tin hoặc dần trượt khỏi mục tiêu ban đầu của người dùng.
2. **Làm việc thừa ($W > 0$ - Scope Creep)**: Tự ý triển khai thêm những tính năng không được yêu cầu, làm rối rắm hệ thống.
3. **Lan man & Sáo rỗng**: Trả lời dài dòng, khô khan, lặp lại các câu chữ khuôn mẫu, tiêu tốn token vô ích.
4. **Code tiềm ẩn lỗi logic**: Vi phạm điều kiện biên, độ phức tạp chu trình quá cao, mã nguồn khó bảo trì.

`mcp-plugin-math` giải quyết triệt để bài toán này bằng cách đặt một **Cổng Thẩm Định Toán Học (Mathematical Metacognition Gate)** ngay trước chu trình tư duy của AI. Trước khi gửi câu trả lời hoặc xuất mã nguồn, AI bắt buộc phải gọi MCP để đối soát với các định lý toán học.

---

## 🔬 4 Trụ Cột Toán Học Cốt Lõi

### 1. Quản Lý Kế Hoạch Bằng Đồ Thị Có Hướng Không Chu Trình (DAG)
Mọi kế hoạch thực thi được mô hình hóa dưới dạng một DAG $G = (V, E)$.
- **Tỷ lệ bao phủ kế hoạch (Coverage Ratio)**:
  $$C = \frac{|V_{\text{exec}} \cap V_{\text{plan}}|}{|V_{\text{plan}}|}$$
  Phải đạt $1.0$ ($100\%$) khi hoàn thành dự án.
- **Định lượng việc làm thừa (Waste / Scope Creep)**:
  $$W = |V_{\text{exec}} \setminus V_{\text{plan}}|$$
  Bất kỳ hành động nào nằm ngoài danh sách đã phê duyệt đều ngay lập tức bị gắn cờ vi phạm ($W > 0$).
- **Tính bất biến Topo (Topological Invariants)**: Đảm bảo các tác vụ tiên quyết bắt buộc phải hoàn thành trước tác vụ phụ thuộc. Tự động phát hiện lỗi vòng lặp (Cycle Detection).

### 2. Lý Thuyết Thông Tin & Mật Độ Ngôn Ngữ (Information Theory)
- **Shannon Entropy**:
  $$H(X) = -\sum_{x} p(x) \log_2 p(x)$$
- **Mật độ thông tin**: $D = H(X) \times \text{TTR}$ (với TTR là Type-Token Ratio - tỷ lệ từ vựng độc bản). Phát hiện các câu từ sáo rỗng và câu văn lặp ý.
- **Ước lượng độ phức tạp Kolmogorov**: Đo tỷ lệ nén (Compression Ratio qua Gzip). Tỷ lệ nén quá thấp trên văn bản dài cảnh báo văn phong nhiều từ đệm (filler tokens).
- **Thước đo khả năng đọc**: Tích hợp Flesch Reading Ease & Gunning Fog Index để kiểm soát, tránh câu văn quá khô khan hoặc quá rối rắm.

### 3. Phân Tích AST & Chất Lượng Mã Nguồn (Code Metrics)
- **Độ phức tạp chu trình McCabe (Cyclomatic Complexity)**:
  $$M = E - N + 2P$$
  Cảnh báo nghiêm ngặt nếu hàm có $M > 10$.
- **Hệ thống đo lường Halstead**: Tính toán Dung lượng ($V$), Độ khó ($D$), Công sức lập trình ($E$) và Số lỗi dự báo ($B = \frac{V}{3000}$).
- **Chỉ số bảo trì phần mềm (Maintainability Index - MI)**:
  $$\text{MI} = 171 - 5.2 \ln(V) - 0.23 M - 16.2 \ln(\text{LOC})$$
  Quy chuẩn: $MI \ge 65$ (Vùng Xanh - Đạt chuẩn sản xuất).
- **Phân tích Cây cú pháp Trừu tượng (AST Tree-sitter)**: Phân tích cú pháp gốc cho **Rust**, **TypeScript/JavaScript**, và **Python**. Tự động đếm node lỗi cú pháp (`node.is_error()`).
- **Phát hiện rủi ro biên**: Cảnh báo unhandled `.unwrap()`, mảng chưa kiểm tra độ dài đã truy xuất index `[0]`, bare `except:`, kiểu dữ liệu lỏng lẻo `any`.

### 4. Lý Thuyết Tập Hợp & Kiểm Tra Mâu Thuẫn Logic
- **Độ thiếu sót yêu cầu**: $\Delta_{\text{missing}} = R_{\text{req}} \setminus R_{\text{impl}}$.
- **Phi mâu thuẫn hình thức (Non-Contradiction)**: Bắt lỗi nếu xuất hiện hai chỉ thị trái ngược nhau cùng tồn tại ($P \wedge \neg P \models \bot$), ví dụ: vừa yêu cầu "không build local" vừa chạy lệnh `cargo build`.

---

## 🛠️ Danh Sách MCP Tools Cung Cấp

| Tool Name | Loại | Chức Năng |
|---|---|---|
| **`math_audit_cognition`** | **Unified Gate** | Kiểm toán toán học toàn diện trong 1 lệnh gọi duy nhất (Yêu cầu + Kế hoạch + Phản hồi nháp + Đoạn mã). Trả về phán quyết `PASS`/`FAIL` và danh sách khuyến nghị khắc phục cụ thể. |
| **`math_track_dag`** | Granular | Theo dõi và cập nhật tiến trình trên đồ thị kế hoạch, phát hiện vi phạm thứ tự thực thi và phát hiện việc làm thừa. |
| **`math_eval_code`** | Granular | Tính toán AST, McCabe Cyclomatic, Halstead, Maintainability Index và các cảnh báo điều kiện biên cho code. |
| **`math_eval_text`** | Granular | Đo lường Shannon entropy, mật độ thông tin, tỷ lệ nén và độ dễ hiểu của văn bản. |
| **`math_verify_constraints`** | Granular | Đối soát tập hợp yêu cầu của người dùng với các tuyên bố thực hiện thực tế. |

---

## 🚀 Cài Đặt & Tích Hợp

### Tải File Thực Thi Đã Build Sẵn (Pre-built Binaries)
File thực thi standalone cho **Linux (x86_64)** và **Windows (x86_64)** được tự động biên dịch và kiểm thử qua [GitHub Actions](https://github.com/Nhan-209/mcp-plugin-math/actions).

### Cấu Hình Vào MCP Client

Thêm vào file cấu hình `claude_desktop_config.json` hoặc Antigravity / Gemini CLI:

```json
{
  "mcpServers": {
    "math-verifier": {
      "command": "/duong/dan/toi/mcp-plugin-math",
      "args": []
    }
  }
}
```

Trên Windows:
```json
{
  "mcpServers": {
    "math-verifier": {
      "command": "C:\\duong\\dan\\toi\\mcp-plugin-math.exe",
      "args": []
    }
  }
}
```

---

## 🤖 Bộ Rule & Skill Cho AI Agent

Dự án bao gồm sẵn bộ quy tắc và kỹ năng tương thích hoàn toàn với các Agentic AI:
- **Quy tắc P0 (`rules/math-verification.md`)**: Bắt buộc AI phải gọi tool `math_audit_cognition` trước khi xuất kết quả cho người dùng.
- **Kỹ năng (`skills/math-metacognition/SKILL.md`)**: Hướng dẫn AI phương pháp mô hình hóa yêu cầu thành DAG và giải mã ma trận số liệu toán học.
- **Bảng tham chiếu ngưỡng (`skills/math-metacognition/references/metric-thresholds.md`)**: Bảng tra cứu các giới hạn chấp nhận được của từng chỉ số.

---

## 📜 Giấy Phép
Dự án được phát hành theo giấy phép mã nguồn mở [MIT License](LICENSE).
