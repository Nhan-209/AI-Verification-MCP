# Math MCP Plugin - Formal Metacognition for AI

## Goal
Xây dựng MCP Server bằng Rust giúp AI tự kiểm chứng nhận thức bằng toán học (Lý thuyết đồ thị DAG, Lý thuyết thông tin Shannon, Độ phức tạp McCabe & Halstead, AST Tree-sitter), tích hợp Skill + Rule, tạo GitHub Public Repo và CI/CD Actions (tuyệt đối không build/test trên máy local).

## Tasks
- [x] Task 1: Khởi tạo kế hoạch chi tiết và thiết lập kiến trúc. → Verify: implementation_plan.md & math-mcp-plugin.md được tạo.
- [x] Task 2: Khởi tạo Git repository và cấu hình `.gitignore` chuẩn Rust. → Verify: `git status` hợp lệ.
- [x] Task 3: Tạo GitHub public repo `Nhan-209/mcp-plugin-math` qua GitHub CLI. → Verify: `gh repo view` trả về repo public.
- [x] Task 4: Thiết lập GitHub Actions CI workflow (`.github/workflows/ci.yml`) cho Rust. → Verify: File workflow đúng cấu trúc YAML.
- [x] Task 5: Viết `Cargo.toml` với đầy đủ dependencies chuẩn xác. → Verify: Cấu hình dependencies hoàn chỉnh.
- [x] Task 6: Viết Engine Toán học (dag.rs, entropy.rs, code_metrics.rs, constraints.rs). → Verify: Mã nguồn hoàn thiện với unit tests.
- [x] Task 7: Viết MCP Protocol Server & Tools (main.rs, protocol.rs, handlers.rs, unified_audit.rs). → Verify: Khung MCP Server sẵn sàng.
- [x] Task 8: Viết Rule & Skill cho Agentic AI (math-verification.md, SKILL.md). → Verify: Agent rules & skills đầy đủ.
- [x] Task 9: Viết `README.md` & `LICENSE` (MIT). → Verify: Tài liệu trực quan, rõ ràng.
- [ ] Task 10: Commit và Push các bản cập nhật định dạng, theo dõi GitHub Actions CI xanh 100%. → Verify: `gh run list` báo success.

## Done When
- [ ] Repo GitHub public `Nhan-209/mcp-plugin-math` hoạt động.
- [ ] GitHub Actions CI chạy pass toàn bộ (clippy, test, release build).
- [ ] Mã nguồn Rust hoàn chỉnh, code clean, kiến trúc module rõ ràng.
- [ ] Bộ Rule + Skill sẵn sàng cho AI sử dụng.
- [ ] Máy thật của người dùng không chạy bất kỳ lệnh `cargo build`/`cargo test` nào.

## Notes
- Giữ nghiêm ngặt quy tắc: Không chạy build/test local. Mọi kiểm thử thực hiện trên GitHub Actions.
