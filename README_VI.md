# 🧮 MCP Plugin Math: Hệ Thống Siêu Nhận Thức Toán Học & Chống Ảo Giác Cho AI

[English](README.md) | [Tiếng Việt](README_VI.md)

[![Rust CI/CD](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml/badge.svg)](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Protocol: MCP 2024-11-05](https://img.shields.io/badge/MCP-2024--11--05-brightgreen.svg)](https://modelcontextprotocol.io)
[![Version: 0.2.0](https://img.shields.io/badge/version-0.2.0-orange.svg)](Cargo.toml)

Một **Model Context Protocol (MCP)** Server hiệu năng cực cao viết bằng **Rust**, giúp các AI Agent triệt tiêu ảo giác (hallucinations), ngăn ngừa lệch phạm vi (scope creep), kiểm soát độ dài dòng, đánh giá mức độ tự tin, phân tích rủi ro suy thoái mã nguồn và đảm bảo mã nguồn đạt chuẩn sản phẩm thông qua **các chứng minh toán học tất định và hệ thống chỉ số phần mềm hình thức**.

---

## 🌟 Triết Lý: Chuyển Từ Dự Đoán Xác Suất Sang Chứng Minh Toán Học

Các Mô hình Ngôn ngữ Lớn (LLM) bản chất là các bộ dự đoán token theo phân phối xác suất. Nếu thiếu điểm tựa toán học tất định, AI thường gặp các vấn đề:
1. **Ảo giác & Lạc đề (Hallucination)**: Tự bịa dữ kiện hoặc trôi dạt xa khỏi ý định ban đầu của người dùng.
2. **Làm thừa việc ($W > 0$)**: Tự ý code những thứ không ai yêu cầu, gây phình to codebase.
3. **Dài dòng & Sáo rỗng**: Trả lời bằng nhiều câu đệm khách sáo, lặp từ, mật độ thông tin nghèo nàn.
4. **Tự tin giả tạo (False Confidence)**: Khẳng định những điều không có cơ sở hoặc dùng quá nhiều từ do dự mơ hồ.
5. **Rủi ro hồi quy mã nguồn (Regressions)**: Thay đổi code làm tăng đột biến độ phức tạp hoặc phát sinh lỗi tiềm ẩn.

`mcp-plugin-math` đóng vai trò là một **Cổng Siêu Nhận Thức Toán Học (Mathematical Metacognition Gate)**. Trước khi phản hồi hoặc bàn giao mã, AI sẽ tự kiểm toán hành vi của mình qua các mô hình toán học chặt chẽ.

---

## 🔬 Các Trụ Cột Toán Học

### 1. Kiểm Toán Kế Hoạch Qua Đồ Thị Có Hướng Không Chu Trình (DAG)
Mọi kế hoạch thực thi được mô hình hóa thành một DAG $G = (V, E)$.
- **Tỷ lệ bao phủ kế hoạch (Coverage Ratio)**:
  $$C = \frac{|V_{\text{thực\_thi}} \cap V_{\text{kế\_hoạch}}|}{|V_{\text{kế\_hoạch}}|}$$
- **Chỉ số lãng phí / Làm thừa (Scope Creep)**:
  $$W = |V_{\text{thực\_thi}} \setminus V_{\text{kế\_hoạch}}|$$
  Bất kỳ hành động nào nằm ngoài kế hoạch đã duyệt sẽ kích hoạt ngay **Vi Phạm Phạm Vi ($W > 0$)**.
- **Bất biến tô pô (Topological Invariants)**: Đảm bảo các tác vụ phụ thuộc chỉ được phép thực hiện khi tác vụ tiên quyết đã hoàn tất.

### 2. Lý Thuyết Thông Tin & Mật Độ Ngôn Ngữ
- **Shannon Entropy**:
  $$H(X) = -\sum_{x} p(x) \log_2 p(x)$$
- **Mật độ thông tin**: $D = H(X) \times \text{TTR}$ (Type-Token Ratio). Phát hiện văn bản sáo rỗng hoặc lặp lại.
- **Xấp xỉ độ phức tạp Kolmogorov**: Tỷ lệ nén qua thuật toán Gzip lọc bỏ các nội dung dư thừa, ít thông tin.
- **Nhận diện cụm từ đệm AI & Ước tính độ dài tối ưu**: Phát hiện câu chào đệm khách sáo ("As an AI...", "I'd be happy to...") và tính toán độ dài văn bản tối ưu dựa trên điểm bão hòa entropy.
- **Chỉ số khả năng đọc**: Flesch Reading Ease và Gunning Fog Index kiểm soát văn bản luôn rõ ràng, dễ hiểu.

### 3. Đánh Giá Mức Độ Tự Tin Siêu Nhận Thức (Metacognitive Confidence)
- **Tỷ lệ do dự (Hedging Ratio)**: Đo lường tần suất xuất hiện các từ ngữ mơ hồ (*maybe*, *probably*, *I think*, *might*).
- **Mật độ khẳng định (Assertion Density)**: $A = 1 - H$, thể hiện các khẳng định chắc chắn, có căn cứ.
- **Chỉ số cụ thể (Specificity Score)**: Đo lường sự hiện diện của dẫn chứng thực nghiệm (đoạn mã, đường dẫn tệp, số liệu, URL).
- **Tự mâu thuẫn nội tại**: Tự động phát hiện các phát biểu xung đột lẫn nhau ngay trong câu trả lời.

### 4. Độ Phức Tạp Phần Mềm, AST & Phân Tích Khác Biệt (Diff)
- **Độ phức tạp Cyclomatic McCabe**: $M = E - N + 2P$. Cảnh báo khi mật độ rẽ nhánh quá cao ($M > 20$).
- **Chỉ số Halstead**: Đo lường Thể tích chương trình ($V$), Độ khó ($D$), Nỗ lực lập trình ($E$) và Số lỗi ước tính ($B = \frac{V}{3000}$).
- **Chỉ số khả năng bảo trì (Maintainability Index - MI)**:
  $$\text{MI} = 171 - 5.2 \ln(V) - 0.23 M - 16.2 \ln(\text{LOC})$$
- **Phân tích Cây cú pháp Trừu tượng (AST Tree-sitter)**: Hỗ trợ đa ngôn ngữ qua Cargo feature flags (**Rust**, **TypeScript/JavaScript**, **Python**, **Go**, **Java**, **C**, **C++**).
- **Phân tích rủi ro suy thoái (Diff Analysis via LCS)**: So sánh sự thay đổi code trước/sau qua dãy con chung dài nhất, tính toán biến thiên độ phức tạp ($\Delta M$, $\Delta\text{MI}$), các hàm bị ảnh hưởng và chỉ số rủi ro tổng hợp.
- **Phát hiện rủi ro biên**: Cảnh báo unhandled `.unwrap()`, mảng chưa kiểm tra độ dài đã truy xuất `[0]`, bare `except:`, kiểu dữ liệu lỏng lẻo `any`, rò rỉ bộ nhớ `malloc` thiếu `free`, hàm C không an toàn.

### 5. Lý Thuyết Tập Hợp & Khớp Ngữ Nghĩa N-gram
- **Khớp ràng buộc ngữ nghĩa (Semantic Matching)**: Ứng dụng hệ số tương đồng Jaccard trên n-gram ký tự $J(A, B) = \frac{|A \cap B|}{|A \cup B|}$ kết hợp đối soát từ vựng để nhận diện yêu cầu được thỏa mãn, không phụ thuộc vào chuỗi con thô.
- **Phát hiện mâu thuẫn phủ định động**: Tự động phân tích các cấu trúc phủ định ("no X", "without X", "must not X") để tìm kiếm các mâu thuẫn logic phát sinh ($P \wedge \neg P \models \bot$).

---

## 🛠️ Danh Sách MCP Tools Cung Cấp

| Tool Name | Loại | Chức Năng |
|---|---|---|
| **`math_audit_cognition`** | **Unified Gate** | Cổng kiểm toán siêu nhận thức toàn diện (Yêu cầu + Kế hoạch + Phản hồi nháp + Tự tin + Mã nguồn) với thuật toán tính điểm trọng số. Trả về phán quyết `PASS`/`FAIL` và danh sách khuyến nghị khắc phục. |
| **`math_track_dag`** | Granular | Theo dõi và cập nhật tiến trình trên đồ thị DAG, phát hiện vi phạm thứ tự tô pô, chu trình và việc làm ngoài kế hoạch. |
| **`math_eval_code`** | Granular | Phân tích AST, McCabe Cyclomatic, Halstead, Maintainability Index và các cảnh báo biên cho nhiều ngôn ngữ lập trình. |
| **`math_eval_diff`** | Granular | Phân tích khác biệt mã nguồn trước/sau (LCS), đo lường tỷ lệ thay đổi, biến thiên độ phức tạp ($\Delta M$, $\Delta\text{MI}$), phạm vi hàm bị ảnh hưởng và điểm rủi ro hồi quy. |
| **`math_eval_text`** | Granular | Đo lường Shannon entropy, mật độ thông tin, tỷ lệ nén, chỉ số khả năng đọc, phát hiện từ đệm AI và độ dài tối ưu. |
| **`math_confidence`** | Granular | Đánh giá mức độ tự tin nhận thức, tỷ lệ ngập ngừng do dự, mật độ khẳng định, tính cụ thể và phát hiện tự mâu thuẫn trong văn bản. |
| **`math_verify_constraints`** | Granular | Đối soát tập hợp yêu cầu với giải pháp thực tế bằng tương đồng Jaccard n-gram và phát hiện mâu thuẫn phủ định động. |

---

## 📦 Hỗ Trợ Ngôn Ngữ & Feature Flags

`mcp-plugin-math` sử dụng Cargo feature flags để tối ưu dung lượng tệp thực thi:

| Feature Flag | Ngôn Ngữ Hỗ Trợ | Mặc Định? |
|---|---|:---:|
| `lang-rust` | Rust | ✅ |
| `lang-typescript` | TypeScript, JavaScript, TSX | ✅ |
| `lang-python` | Python | ✅ |
| `lang-go` | Go | Tùy chọn |
| `lang-java` | Java | Tùy chọn |
| `lang-c` | C | Tùy chọn |
| `lang-cpp` | C++ | Tùy chọn |
| `all-languages` | Toàn bộ 7 ngôn ngữ | Tùy chọn |

Để biên dịch bản hỗ trợ đầy đủ tất cả các ngôn ngữ:
```bash
cargo build --release --features all-languages
```

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
- **Kỹ năng (`skills/math-metacognition/SKILL.md`)**: Hướng dẫn AI cách thiết lập đồ thị DAG, phân tích chỉ số và sửa chữa các vi phạm toán học.

---

## 📜 Giấy Phép
Phát hành theo [Giấy phép MIT](LICENSE).
