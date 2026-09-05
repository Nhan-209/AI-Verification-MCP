# 🧮 MCP Plugin Math: Hệ Thống Siêu Nhận Thức Toán Học & Chống Ảo Giác Cho AI

[English](README.md) | [Tiếng Việt](README_VI.md)

[![Rust CI/CD](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml/badge.svg)](https://github.com/Nhan-209/mcp-plugin-math/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Protocol: MCP 2024-11-05](https://img.shields.io/badge/MCP-2024--11--05-brightgreen.svg)](https://modelcontextprotocol.io)
[![Version: 0.3.0](https://img.shields.io/badge/version-0.3.0-orange.svg)](Cargo.toml)

Một **Model Context Protocol (MCP)** Server hiệu năng cực cao viết bằng **Rust**, giúp các AI Agent triệt tiêu ảo giác (hallucinations), ngăn ngừa lệch phạm vi (scope creep), kiểm soát độ dài dòng, hiệu chuẩn mức độ tự tin (anti-overconfidence), bắt buộc nghiên cứu thực nghiệm (Research Gate), chủ động lường trước lỗi và ca biên (Foresight Engine) thông qua **các chứng minh toán học tất định và hệ thống chỉ số phần mềm hình thức**.

---

## 🌟 Triết Lý: Chuyển Từ Dự Đoán Xác Suất Sang Chứng Minh Toán Học

Các Mô hình Ngôn ngữ Lớn (LLM) bản chất là các bộ dự đoán token theo phân phối xác suất. Nếu thiếu điểm tựa toán học tất định, AI thường gặp các vấn đề:
1. **Ảo giác & Lạc đề (Hallucination)**: Tự bịa dữ kiện hoặc trôi dạt xa khỏi ý định ban đầu của người dùng.
2. **Làm thừa việc ($W > 0$)**: Tự ý code những thứ không ai yêu cầu, gây phình to codebase.
3. **Dài dòng & Sáo rỗng**: Trả lời bằng nhiều câu đệm khách sáo, lặp từ, mật độ thông tin nghèo nàn.
4. **Tự tin thái quá (Overconfidence)**: Tuyên bố "chắc chắn 100%", "đảm bảo không lỗi" khi không hề có dữ liệu thực nghiệm.
5. **Lười nghiên cứu (Research Deficit)**: Đoán mò tính năng thư viện, phiên bản, thông số thay vì trích dẫn tài liệu chính thống.
6. **Lập kế hoạch nông cạn (Lazy Plan)**: Chỉ nghĩ đến trường hợp lý tưởng (happy path), bỏ qua xử lý lỗi, trường hợp biên và kiểm thử.

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

### 2. Lý Thuyết Thông Tin & Tách Câu Thông Minh Song Ngữ
- **Shannon Entropy**:
  $$H(X) = -\sum_{x} p(x) \log_2 p(x)$$
- **Mật độ thông tin**: $D = H(X) \times \text{TTR}$ (Type-Token Ratio).
- **Bộ tách câu thông minh (`text_utils.rs`)**: Bảo toàn trọn vẹn URL (`github.com/...`), phiên bản (`v1.2.3`), số thập phân (`3.14`), chữ viết tắt (`e.g.`, `vd.`), không gây vụn vặt câu văn.
- **Từ điển câu đệm AI song ngữ EN/VI**: Nhận diện câu đệm khách sáo trong cả tiếng Anh và tiếng Việt.

### 3. Hiệu Chuẩn Nhận Thức & Chống Tự Tin Thái Quá (Epistemic Calibration)
- **Chỉ số hiệu chuẩn (Calibration Index)**: Phạt nặng các khẳng định tuyệt đối hóa ("chắc chắn 100%", "đảm bảo hoàn hảo", "guaranteed") nếu không có dữ liệu thực nghiệm đi kèm.
- **Phân loại nhận thức**:
  - `CALIBRATED`: Tự tin trên cơ sở thực nghiệm, lập luận vững chắc.
  - `OVERCONFIDENT`: Khẳng định bừa bãi, dùng từ ngữ tuyệt đối hóa nhưng thiếu bằng chứng.
  - `UNDERCONFIDENT`: Hàm lượng kỹ thuật cao nhưng do dự không cần thiết.
  - `EVASIVE`: Trả lời né tránh, vòng vo, toàn câu đệm mơ hồ.

### 4. Cổng Kiểm Toán Nghiên Cứu Thực Nghiệm (Research Gate)
- **Tỷ lệ dẫn chứng (Evidence Ratio)**: $E = \frac{\text{citations}}{\max(\text{factual\_claims}, 1)}$.
- **Phát hiện thiếu hụt nghiên cứu (`RESEARCH_DEFICIT`)**: Nếu đưa ra các khẳng định về phiên bản, hiệu năng, API mà không có trích dẫn nguồn (URL, RFC, đường dẫn file, log kiểm thử) → Ép AI phải dừng lại tra cứu trước khi trả lời.

### 5. Tư Duy Tiên Liệu & Kỹ Thuật Phòng Ngừa (Foresight Engine)
- **Thiết kế phòng thủ**: Kiểm tra sự hiện diện của cơ chế xử lý lỗi, timeout, fallback, retry logic.
- **Bao phủ trường hợp biên**: Đánh giá xử lý mảng rỗng, giá trị tới hạn, tràn số, concurrency.
- **Chống kế hoạch lười (`LAZY_PLAN`)**: Đối soát độ sâu kế hoạch với độ phức tạp của bài toán, ngăn ngừa việc lập kế hoạch 1 bước sơ sài.
- **Độ phức tạp Cyclomatic McCabe**: $M = E - N + 2P \le 10$.
- **Chỉ số bảo trì (Maintainability Index - MI)**: $MI \ge 65$.
- **Phân tích rủi ro suy thoái (Diff Analysis via LCS)**: Đo lường biến thiên độ phức tạp trước khi commit code.

### 6. Lý Thuyết Tập Hợp & Khớp Ngữ Nghĩa N-gram
- **Khớp ràng buộc ngữ nghĩa (Semantic Matching)**: Tương đồng Jaccard trên n-gram ký tự $J(A, B) = \frac{|A \cap B|}{|A \cup B|}$ kết hợp đối soát từ vựng.
- **Phát hiện mâu thuẫn phủ định động**: Tự động phân tích các cấu trúc phủ định ("không được X", "without X", "must not X") để tìm mâu thuẫn logic ($P \wedge \neg P \models \bot$).

---

## 🛠️ Danh Sách 9 MCP Tools Cung Cấp

| Tool Name | Loại | Chức Năng |
|---|---|---|
| **`math_audit_cognition`** | **Unified Gate** | Kiểm toán 6 trụ cột (Yêu cầu + Kế hoạch + Hiệu chuẩn nhận thức + Nghiên cứu + Tiên liệu + Code). Trả về phán quyết `PASS`/`FAIL` và danh sách khuyến nghị cụ thể. |
| **`math_track_dag`** | Granular | Theo dõi và cập nhật tiến trình trên đồ thị DAG, phát hiện vi phạm thứ tự tô pô, chu trình và việc làm thừa. |
| **`math_eval_code`** | Granular | Phân tích AST, McCabe Cyclomatic, Halstead, Maintainability Index và các cảnh báo biên cho 7 ngôn ngữ. |
| **`math_eval_diff`** | Granular | Phân tích khác biệt mã nguồn (LCS), đo lường tỷ lệ thay đổi, biến thiên độ phức tạp ($\Delta M$, $\Delta\text{MI}$), danh sách hàm bị ảnh hưởng và điểm rủi ro hồi quy. |
| **`math_eval_text`** | Granular | Đo lường Shannon entropy, mật độ thông tin, tỷ lệ nén, chỉ số khả năng đọc, phát hiện từ đệm AI và độ dài tối ưu. |
| **`math_confidence`** | Granular | Cổng hiệu chuẩn nhận thức. Đánh giá mức độ tự tin thái quá, tuyên bố số liệu bừa bãi, tỷ lệ do dự và tự mâu thuẫn. |
| **`math_audit_research`** | Granular | Ép AI nghiên cứu thực nghiệm. Rà soát các khẳng định kỹ thuật, kiểm tra trích dẫn (URL, RFC, file path, test log) và cảnh báo thiếu hụt nghiên cứu. |
| **`math_eval_foresight`** | Granular | Đánh giá tư duy tiên liệu: xử lý lỗi phòng thủ, ca biên, chiến lược kiểm thử và phát hiện kế hoạch sơ sài (lazy plan). |
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
- **Kỹ năng (`skills/math-metacognition/SKILL.md`)**: Hướng dẫn AI cách thiết lập đồ thị DAG, hiệu chuẩn nhận thức, tra cứu nghiên cứu và lường trước lỗi tương lai.

---

## 📜 Giấy Phép
Phát hành theo [Giấy phép MIT](LICENSE).
