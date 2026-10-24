# Decisions log
Append only. Newest at the bottom. Never edit or delete an entry: to change a decision, add a new entry that supersedes it.

Format:
## YYYY-MM-DD — short title
- Topic: <slug of the part of the system this decides, e.g. auth-method, dataset-split>
- Decision: what we decided, in one clear statement
- Why: the forces behind it
- Who / task: the person and the plan.csv task id
- Supersedes: <date — title of the entry this replaces> (only when it replaces one)
- Result: <run-id · config · metrics · artifact path> (only for the chosen result of an experiment)

## 2026-10-24 — Không dùng file cấu hình, hard-code trong source
- Topic: configuration
- Decision: Không đọc config từ file (TOML/JSON) ở runtime. Danh sách game cần chặn, kênh thông báo, webhook URL... đều là hằng số khai báo trong source (dự kiến `src/settings.rs`). Đổi cấu hình = sửa code + build lại.
- Why: Ưu tiên giữ chương trình đơn giản — tải `.exe` về chạy luôn, không cần người dùng tự tạo/sửa file cấu hình. Ứng dụng chỉ một người dùng (tác giả) chỉnh sửa.
- Who / task: Duc42195 / GA-2

## 2026-10-24 — Kênh thông báo đầu tiên: Discord Webhook
- Topic: notification-channel
- Decision: Dùng Discord Webhook làm kênh thông báo mặc định cho MVP (qua trait `Notifier`). Discord Bot (2 chiều) để dành làm sau, không ưu tiên ngay. Chỉ chạy 1 kênh active tại một thời điểm, không cần cơ chế fallback nhiều kênh song song.
- Why: Webhook đơn giản nhất (không cần quản lý token/giữ bot online), đủ cho mục tiêu gửi cảnh báo 1 chiều. Bot chỉ cần thiết nếu muốn tương tác 2 chiều (chưa có nhu cầu). Giữ interface `Notifier` generic để sau này thêm `HttpNotifier` gửi về server riêng mà không đổi code chỗ khác.
- Who / task: Duc42195 / GA-7, GA-10

## 2026-10-24 — Giữ polling tối ưu thay vì event-driven (ETW/WMI)
- Topic: process-detection
- Decision: Giữ cơ chế polling (quét tiến trình định kỳ mỗi 1-2 giây) nhưng tách luồng "phát hiện + chặn" khỏi "gửi thông báo" bằng channel, thay vì chuyển sang event-driven qua ETW/WMI.
- Why: Event-driven cho độ trễ gần 0 nhưng phức tạp hơn nhiều (API Windows nâng cao, luôn cần quyền admin, khó bảo trì) — không đáng đánh đổi cho nhu cầu cá nhân hiện tại.
- Who / task: Duc42195 / GA-5

## 2026-10-24 — Secrets nạp lúc build qua build.rs, không hard-code trong source
- Topic: secrets-management
- Decision: Ngoại lệ với quyết định "configuration" ở trên — riêng secrets (vd `DISCORD_WEBHOOK_URL`) không hard-code trong `.rs` và cũng không đọc file ở runtime. `build.rs` đọc `.agents/state/secrets.env` (ưu tiên, git-ignored, riêng của project) hoặc `~/secret.env` (fallback, secrets chung của máy, ngoài repo), rồi nhúng giá trị vào binary lúc compile qua `env!()` trong `src/settings.rs`. Thiếu secret thì `cargo build` lỗi ngay, không build ra binary thiếu cấu hình một cách âm thầm.
- Why: Hard-code trực tiếp vào `.rs` thì secret vào git history vĩnh viễn nếu lỡ commit — rủi ro thật nếu repo public. Đọc file ở runtime lại phá vỡ mục tiêu "tải .exe về chạy luôn, không cần set gì" (quyết định ở trên). Nạp lúc build giữ được cả hai: secret không vào git, binary cuối vẫn tự chứa mọi thứ cần thiết để chạy.
- Supersedes: 2026-10-24 — Không dùng file cấu hình, hard-code trong source (chỉ với secrets; `GAME_LIST`/`POLL_INTERVAL` vẫn hard-code như cũ)
- Who / task: Duc42195 / GA-7

## 2026-10-24 — Smoke-test bằng chương trình giả, chạy qua bản copy đổi tên interpreter
- Topic: testing-harness
- Decision: `scripts/game-alert-test.sh`/`.ps1` là chương trình giả (chỉ sleep lặp lại) để tự test pipeline phát hiện→kill→notify. Chỉ active trong `GAME_LIST` khi build với Cargo feature `smoke-test` (`cargo run --features smoke-test`), không ảnh hưởng build release mặc định. Cách chạy đúng: copy chính interpreter (`bash` trên Linux, `powershell.exe`/`pwsh.exe` trên Windows) thành file tên `game-alert-test`/`game-alert-test.exe`, chạy script qua bản copy đó.
- Why: Verify bằng thực nghiệm (không đoán) rằng chạy script trực tiếp qua shebang/`-File` cho tên tiến trình (comm trên Linux, image name trên Windows) là tên **interpreter** (`bash`/`powershell.exe`), không phải tên file script. Nếu đưa thẳng `bash`/`powershell.exe` vào `GAME_LIST` để test sẽ kill nhầm mọi tiến trình bash/PowerShell khác đang chạy trên máy — rủi ro thật, không chỉ lý thuyết. Copy+rename interpreter là cách tiêu chuẩn để có process name tuỳ ý mà không cần compile.
- Who / task: Duc42195 / GA-11

## 2026-10-25 — Nhận diện máy bằng hostname, đăng ký qua hằng số trong source
- Topic: machine-identification
- Decision: Machine id = hostname (`sysinfo::System::host_name()`, không thêm dependency mới). `settings::REGISTERED_MACHINES: &[(&str, &str)]` ánh xạ hostname -> tên thân thiện, hard-code trong source (đúng quy ước "configuration" ở trên). Máy có trong danh sách hiện tên thân thiện trong tin Discord; máy chưa có hiện hostname thô kèm "CHƯA ĐĂNG KÝ" để biết thêm vào rồi build lại — không có luồng "đăng ký online" hay ghi file nào khác.
- Why: Hostname đơn giản nhất, tận dụng dependency sẵn có (`sysinfo`), đủ dùng cho ngữ cảnh gia đình (không có kẻ địch chủ động đổi tên máy để né tránh). Đánh đổi: không thực sự "bất biến" — đổi tên máy thì đổi machine id theo, máy đó coi như "mới", phải đăng ký lại.
- Who / task: Duc42195 / GA-12

## 2026-10-25 — Webhook rỗng hợp lệ lúc build; hỏi + lưu runtime cho bản public
- Topic: secrets-management
- Decision: `DISCORD_WEBHOOK_URL` đổi từ `env!()` (lỗi compile nếu thiếu) sang `option_env!()` (cho phép rỗng). Nếu rỗng lúc chạy (bản build không kèm `secrets.env`, vd binary đính kèm GitHub Release công khai), `src/webhook_setup.rs` đọc `webhook.txt` cạnh file `.exe`; nếu file đó cũng chưa có, hỏi qua console lúc chạy lần đầu rồi lưu vào đó cho các lần sau. Bản build riêng của maintainer (có `secrets.env`) không đổi hành vi — vẫn nhúng sẵn, không hỏi gì.
- Why: Repo public, nên không thể đính kèm `.exe` đã nhúng webhook thật vào GitHub Release (ai tải cũng `strings` lấy ra URL, gửi tin rác vào Discord riêng của maintainer — đã verify bằng thực nghiệm rằng `strings` đọc được chuỗi đó từ binary cũ). Cơ chế hỏi+lưu runtime làm binary public hoàn toàn không chứa secret nào, mỗi người tải về tự nhập webhook CỦA HỌ, lưu cục bộ trên máy họ — không ảnh hưởng tới nhau. Đã verify bằng thực nghiệm cả 2 nhánh: build không có secrets.env → 0 webhook trong binary, hỏi+lưu đúng `webhook.txt`, chạy lại không hỏi lại; build có secrets.env → vẫn nhúng sẵn như cũ.
- Supersedes: 2026-10-24 — Secrets nạp lúc build qua build.rs, không hard-code trong source (vẫn giữ cơ chế build.rs, chỉ nới lỏng "thiếu thì lỗi" thành "thiếu thì cho phép rỗng + hỏi runtime")
- Who / task: Duc42195

## 2026-10-25 — Revert: webhook luôn bắt buộc nhúng lúc build, không phân phối .exe công khai
- Topic: secrets-management
- Decision: Bỏ cơ chế "webhook rỗng hợp lệ + hỏi qua console lúc chạy lần đầu" (xoá `src/webhook_setup.rs`). `DISCORD_WEBHOOK_URL` quay lại `env!()` — thiếu secret thì `cargo build` lỗi ngay, không có cách nào build ra binary thiếu webhook. Không còn đính `.exe` vào GitHub Release (đã gỡ asset khỏi release `v0.2.0`, sửa lại mô tả release). Mỗi file `.exe` gửi cho máy khác đều do maintainer tự build (đã có webhook thật nhúng sẵn) và gửi riêng, không public.
- Why: Chủ project quyết định không cần kịch bản "người lạ tải binary công khai" nữa — tất cả máy dùng app này (Dell, các máy khác) đều do chính chủ project cấu hình và phân phối thủ công. Bỏ bớt 1 tầng cơ chế (prompt+lưu file runtime) giúp đơn giản hơn khi không còn nhu cầu đó, đúng tinh thần "giữ đơn giản" xuyên suốt dự án.
- Supersedes: 2026-10-25 — Webhook rỗng hợp lệ lúc build; hỏi + lưu runtime cho bản public
- Who / task: Duc42195
