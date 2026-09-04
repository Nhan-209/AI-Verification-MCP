# Math MCP Plugin - Formal Metacognition for AI

## Goal
Xây dựng MCP Server bằng Rust giúp AI tự kiểm chứng nhận thức bằng toán học (Lý thuyết đồ thị DAG, Lý thuyết thông tin Shannon, Độ phức tạp McCabe & Halstead, AST Tree-sitter), tích hợp Skill + Rule, tạo GitHub Public Repo và CI/CD Actions (tuyệt đối không build/test trên máy local).

## Tasks
- [x] Task 1: Khởi tạo kế hoạch chi tiết và thiết lập kiến trúc. → Verify: implementation_plan.md & math-mcp-plugin.md được tạo.
- [ ] Task 2: Khởi tạo Git repository và cấu hình `.gitignore` chuẩn Rust. → Verify: `git status` hợp lệ.
- [ ] Task 3: Tạo GitHub public repo `Nhan-209/mcp-plugin-math` qua GitHub CLI. → Verify: `gh repo view` trả về repo public.
- [ ] Task 4: Thiết lập GitHub Actions CI workflow (`.github/workflows/ci.yml`) cho Rust (fmt, clippy, test, build release). → Verify: File workflow đúng cấu trúc YAML.
- [ ] Task 5: Viết `Cargo.toml` với đầy đủ dependencies chuẩn xác. → Verify: Cấu hình dependencies hoàn chỉnh.
- [ ] Task 6: Viết Engine Toán học:
  - `src/engine/dag.rs`: Đồ thị kế hoạch (Plan DAG, topological order, waste $W$, coverage $C$).
  - `src/engine/entropy.rs`: Lý thuyết thông tin (Shannon entropy, density, readability, compression ratio).
  - `src/engine/code_metrics.rs`: AST metrics (McCabe, Halstead, Maintainability Index, boundary conditions).
  - `src/engine/constraints.rs`: Lý thuyết tập hợp & Kiểm tra mâu thuẫn yêu cầu.
  → Verify: Mã nguồn Rust hoàn thiện đầy đủ tests đơn vị đi kèm trong từng module.
- [ ] Task 7: Viết MCP Protocol Server & Tools:
  - `src/mcp/protocol.rs` & `handlers.rs`: Giao thức JSON-RPC stdio.
  - `src/tools/unified_audit.rs`: Tool `math_audit_cognition`.
  - `src/tools/plan_tracker.rs`, `code_evaluator.rs`, `text_evaluator.rs`, `constraint_checker.rs`.
  - `src/main.rs`: Entrypoint.
  → Verify: Khung MCP Server và các công cụ sẵn sàng kết nối.
- [ ] Task 8: Viết Rule & Skill cho Agentic AI:
  - `.agents/rules/math-verification.md`: Rule bắt buộc gọi MCP kiểm chứng trước khi phản hồi.
  - `.agents/skills/math-metacognition/SKILL.md`: Skill hướng dẫn mô hình hóa toán học.
  → Verify: Agent rules & skills được load và tuân thủ chuẩn AG Kit.
- [ ] Task 9: Viết `README.md` & `LICENSE` (MIT) giải thích cơ sở toán học và cách tích hợp. → Verify: Tài liệu trực quan, rõ ràng.
- [ ] Task 10: Commit và Push toàn bộ lên GitHub public repository. → Verify: Commit được đẩy thành công lên nhánh `main`.
- [ ] Task 11: Theo dõi GitHub Actions CI chạy hoàn tất và xanh 100%. → Verify: `gh run list` báo success, không chạy lệnh build nào ở máy thật.

## Done When
- [ ] Repo GitHub public `Nhan-209/mcp-plugin-math` hoạt động.
- [ ] GitHub Actions CI chạy pass toàn bộ (clippy, test, release build).
- [ ] Mã nguồn Rust hoàn chỉnh, code clean, kiến trúc module rõ ràng.
- [ ] Bộ Rule + Skill sẵn sàng cho AI sử dụng.
- [ ] Máy thật của người dùng không chạy bất kỳ lệnh `cargo build`/`cargo test` nào.

## Notes
- Giữ nghiêm ngặt quy tắc: Không chạy build/test local. Mọi kiểm thử thực hiện trên GitHub Actions.
