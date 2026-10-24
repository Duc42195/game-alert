# game-alert

Ứng dụng nền (background service) viết bằng Rust, chạy trên Windows, giám sát các tiến
trình game đã đăng ký. Khi phát hiện một game đang chạy, ứng dụng chặn tiến trình đó và
gửi cảnh báo qua Discord.

## Trạng thái hiện tại (v0.2.0)

- Quét tiến trình mỗi 2 giây (polling) bằng crate [`sysinfo`](https://crates.io/crates/sysinfo), so khớp tên tiến trình chính xác với danh sách đã đăng ký (mặc định: Roblox, `RobloxPlayerBeta.exe`).
- Phát hiện là kill ngay. Không gửi lặp tin cảnh báo khi cùng một game bị kill nhiều lần liên tiếp (chỉ báo khi chuyển từ idle sang blocked) — xem `docs/design.md`.
- Gửi cảnh báo qua Discord Webhook, kèm tên máy gửi cảnh báo (xem mục Nhận diện máy).
- Việc gửi Discord chạy ở thread riêng, không làm chậm vòng quét chính kể cả khi mạng chậm/lỗi.
- **Nhận diện máy**: tin cảnh báo kèm tên máy. Máy đã đăng ký (trong `REGISTERED_MACHINES`) hiện tên thân thiện; máy chưa đăng ký hiện thẳng hostname kèm "CHƯA ĐĂNG KÝ" để biết thêm vào.

**Chưa có** (xem mục Roadmap): màn hình chặn làm phiền khi phát hiện game (hiện chỉ kill ngầm, không có overlay), Discord Bot (tương tác 2 chiều).

Chi tiết kiến trúc: xem [`docs/design.md`](docs/design.md).

## Yêu cầu

- Rust toolchain khá mới (dự án dùng `edition = "2024"`), nên chạy `rustup update` trước khi build.
- Ứng dụng chỉ nhắm tới Windows (dùng để kill tiến trình, cần chạy trên máy Windows, hoặc build sẵn `.exe` từ máy khác rồi copy sang).

## Cấu hình Discord webhook trước khi build (chỉ cần cho bản tự build riêng)

Dự án không dùng file cấu hình thông thường — mọi tùy chỉnh (danh sách game, máy đã đăng ký) nằm thẳng trong `src/settings.rs`, đổi là build lại. Riêng **webhook Discord** là secret, không được hard-code vào `.rs`/commit lên git, nên xử lý theo 2 cơ chế song song:

1. **Bản bạn tự build cho mình**: tạo file `.agents/state/secrets.env` (ưu tiên, git-ignored) hoặc `~/secret.env` (dùng chung cho máy) với nội dung:
   ```
   DISCORD_WEBHOOK_URL=https://discord.com/api/webhooks/...
   ```
   `build.rs` tự đọc và nhúng vào binary lúc `cargo build` — chạy lên không hỏi gì cả. Cách tạo webhook: xem `docs/design.md` mục "Thiết kế kênh thông báo".

2. **Bản tải từ GitHub Release (không kèm secret nào)**: chạy lần đầu, chương trình tự hỏi qua console "Dán Discord Webhook URL rồi nhấn Enter", lưu vào file `webhook.txt` cạnh `.exe`. Các lần chạy sau không hỏi lại. Muốn đổi webhook: xoá `webhook.txt`, chạy lại, nhập webhook mới.

## Cài đặt / Build

### Build trực tiếp trên Windows

```powershell
cargo build --release
```

Binary nằm tại `target/release/game-alert.exe`.

### Cross-compile từ Linux sang Windows

Trên máy phát triển (Linux), cài `mingw-w64` (cung cấp linker cho target GNU) rồi thêm target Windows:

```bash
sudo apt-get install -y mingw-w64
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

Binary sẽ nằm tại `target/x86_64-pc-windows-gnu/release/game-alert.exe`, copy file này sang máy Windows để chạy.

### Chạy

Chạy trực tiếp file `.exe` trên Windows. Vì ứng dụng cần kill tiến trình của chương trình khác, có thể cần chạy với quyền Administrator ("Run as administrator") nếu Windows từ chối quyền kill. Chương trình chạy vô hạn (vòng lặp quét liên tục) — dừng bằng cách đóng cửa sổ console hoặc `Ctrl+C`.

## Tự test (không cần Roblox thật)

```bash
cargo run --features smoke-test
```
Bật thêm một "game giả" (`game-alert-test`, chỉ sleep lặp lại, không làm gì) vào danh sách cần chặn — không ảnh hưởng tới build release mặc định. Cách chạy chương trình giả đúng cách (quan trọng: không chạy `.sh`/`.ps1` trực tiếp): xem [`scripts/README.md`](scripts/README.md).

## Roadmap

- [ ] Hiện màn hình chặn (overlay) làm phiền khi phát hiện game, thay vì chỉ kill ngầm.
- [ ] Discord Bot cho tương tác 2 chiều (vd lệnh tạm mở chặn).

Chi tiết các quyết định thiết kế: [`docs/design.md`](docs/design.md), lịch sử quyết định: [`.agents/wiki/decisions-log.md`](.agents/wiki/decisions-log.md).
