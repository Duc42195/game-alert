# Smoke test — "game" giả để tự test

`game-alert-test.sh` / `game-alert-test.ps1` là một chương trình không làm gì cả (chỉ
sleep lặp lại), dùng để test `game-alert` thật (phát hiện → kill → gửi Discord) mà
không cần mở Roblox. Chỉ active khi build với feature `smoke-test` — build release
bình thường **không** đụng tới mấy file này (xem `src/settings.rs`).

## Build bản smoke-test

```bash
cargo run --features smoke-test
# hoặc build sẵn:
cargo build --release --features smoke-test
```

## Linux / máy dev

Chạy script trực tiếp sẽ cho tên tiến trình là `bash` (đã verify bằng thực nghiệm —
không phải "game-alert-test" như hay bị đồn). Thêm `bash` vào `GAME_LIST` sẽ kill mọi
bash khác đang chạy trên máy — nguy hiểm. Cách đúng: copy chính `bash` thành file tên
`game-alert-test`, rồi chạy script bằng bản copy đó (chỉ cần làm 1 lần):

```bash
cp /bin/bash ./scripts/game-alert-test
./scripts/game-alert-test ./scripts/game-alert-test.sh &
```
`ps -p <pid> -o comm` sẽ ra đúng `game-alert-test`, khớp với `GAME_LIST` ở chế độ
`smoke-test`. File `game-alert-test` (copy của bash, không có đuôi `.sh`) nên được
`.gitignore` — đã thêm sẵn.

## Windows

PowerShell chạy `.ps1` trực tiếp sẽ hiện tên tiến trình là `powershell.exe` — nếu thêm
tên đó vào `GAME_LIST` thì sẽ kill nhầm mọi cửa sổ PowerShell khác đang mở. Cách đúng:
tạo một bản sao của PowerShell mang tên `game-alert-test.exe`, rồi luôn khởi động
script bằng bản sao đó (chỉ cần làm 1 lần, file copy giữ lại dùng nhiều lần):

```powershell
# Windows PowerShell 5.1 (powershell.exe) có sẵn trên mọi máy Windows:
Copy-Item "$PSHOME\powershell.exe" ".\game-alert-test.exe"

# Hoặc nếu bạn cài PowerShell 7 (pwsh.exe):
# Copy-Item (Get-Command pwsh).Source ".\game-alert-test.exe"

# Chạy "game" giả — Task Manager sẽ thấy đúng tên game-alert-test.exe:
.\game-alert-test.exe -NoLogo -File .\game-alert-test.ps1
```

## Kỳ vọng khi chạy game-alert (bản smoke-test) song song

- Trong ~`POLL_INTERVAL` (mặc định 2 giây), tiến trình giả bị kill, log in ra
  "Đã chặn tiến trình: game-alert-test...".
- Một tin nhắn cảnh báo xuất hiện trong channel Discord đã cấu hình.
- Chạy lại tiến trình giả (script/copy exe) liên tục → vẫn bị kill mỗi lần, nhưng
  **không** có thêm tin nhắn Discord mới cho tới khi nó thật sự idle rồi chạy lại
  (xem GA-4 trong `plan.csv` / `docs/design.md`).

Nhớ dừng tiến trình `game-alert` (Ctrl+C) sau khi test xong — bản `smoke-test` vẫn kill
Roblox thật nếu nó đang chạy, vì `GAME_LIST` ở chế độ này vẫn giữ `RobloxPlayerBeta.exe`.
