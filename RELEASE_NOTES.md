# Release Notes

## Unreleased

- **Đổi hướng phân phối** (revert một phần của v0.2.0): bỏ cơ chế "webhook rỗng hợp lệ +
  hỏi qua console lúc chạy lần đầu" (`webhook_setup.rs`). `DISCORD_WEBHOOK_URL` quay lại
  bắt buộc nhúng lúc build (`cargo build` lỗi ngay nếu thiếu secret) — không còn publish
  `.exe` công khai lên GitHub Release nữa; mỗi file `.exe` (đã có webhook thật nhúng sẵn)
  do maintainer tự build và gửi riêng cho từng máy.

Dự kiến cho các bản tiếp theo:

- Thêm màn hình chặn (overlay) làm phiền khi phát hiện game đang chạy, thay vì chỉ kill ngầm.
- Chạy nền không mở cửa sổ console (Windows subsystem riêng).
- Discord Bot cho tương tác 2 chiều (vd lệnh tạm mở chặn trong X phút).

## v0.2.0 — 2026-10-25

Pipeline hoạt động đầy đủ, test thật trên máy dev (Linux) và sẵn sàng build cho Windows.

- Tích hợp logic quét/kill tiến trình (`block-game.rs` cũ) vào `src/`, tách thành module
  riêng: `process_monitor` (vòng quét + state idle/blocked, tránh gửi lặp tin khi cùng
  một game bị kill liên tục), `blocker` (kill), `notifier` (gửi Discord).
- Tách luồng xử lý: vòng quét tiến trình và việc gửi thông báo chạy độc lập qua channel —
  gửi thông báo không làm chậm việc phát hiện/kill game kế tiếp, kể cả khi mạng chậm/lỗi.
- Thêm `DiscordWebhookNotifier`: gửi cảnh báo qua Discord Webhook. Đã test thật, webhook
  trả về HTTP 204.
- **Nhận diện máy**: tin cảnh báo kèm tên máy — máy đã đăng ký (`REGISTERED_MACHINES`
  trong `settings.rs`) hiện tên thân thiện, máy chưa đăng ký hiện hostname thô.
- **Secrets**: webhook URL nhúng lúc build qua `build.rs` (từ `.agents/state/secrets.env`
  hoặc `~/secret.env`, không bao giờ vào git) cho bản tự build. Bản build công khai (không
  kèm secret nào, ví dụ binary đính ở GitHub Release) có webhook rỗng trong binary — hỏi
  qua console lúc chạy lần đầu, lưu vào `webhook.txt` cạnh `.exe`, không hỏi lại các lần sau.
- Smoke-test harness: `cargo run --features smoke-test` + chương trình giả trong `scripts/`
  để tự test toàn bộ pipeline mà không cần Roblox thật.
- 10 unit test (state tracking, JSON escape, nhận diện máy, đường dẫn file webhook).

## v0.1.0 — 2026-05-10

Phiên bản khởi tạo.

- Thêm logic phát hiện tiến trình game bằng crate `sysinfo`: quét danh sách tiến trình
  đang chạy mỗi 5 giây, so khớp theo tên tiến trình trong danh sách đã đăng ký.
- Hỗ trợ game đầu tiên: Roblox (`RobloxPlayerBeta.exe`).
- Khi phát hiện, kill tiến trình ngay lập tức.
- **Known issue**: logic này hiện nằm ở file `block-game.rs` ở thư mục gốc, **chưa** nằm
  trong `src/`, nên chưa được `cargo build`/`cargo run` chạy tới — `src/main.rs` hiện vẫn
  chỉ là chương trình mẫu `Hello, world!`.
