# Thiết kế hệ thống — game-alert

## Mục tiêu

Ứng dụng nền trên Windows, giám sát các tiến trình game đã đăng ký (bắt đầu với Roblox).
Khi phát hiện một game đang chạy, hệ thống cần:

1. Chặn game đó (hiện tại: kill tiến trình).
2. Hiện màn hình chặn làm phiền, để nhắc người dùng dừng chơi (chưa có).
3. Gửi cảnh báo ra ngoài, ví dụ qua Discord webhook, để người khác (phụ huynh) biết (chưa có).

Phạm vi: dùng cho mục đích cá nhân/gia đình, chạy một máy Windows, không cần hỗ trợ
multi-user hay quản lý tập trung nhiều máy.

## Kiến trúc hiện tại (as-built)

Toàn bộ logic hiện nằm trong một file duy nhất, `block-game.rs` (ở thư mục gốc, **chưa**
được di chuyển vào `src/`, nên chưa thuộc về cargo build target nào):

```
loop {
    system.refresh_processes()              // quét lại danh sách tiến trình
    for game in game_list {                  // game_list = ["RobloxPlayerBeta.exe"]
        if tiến trình "game" đang chạy {
            process.kill()                   // kill ngay
        }
    }
    sleep(5s)
}
```

Đặc điểm:
- Đơn luồng, dùng `sysinfo::System` để đọc danh sách tiến trình, polling mỗi 5 giây.
- Danh sách game (`game_list`) hard-code trong source.
- Chỉ có hành động kill — không có cảnh báo, không có overlay, không có log ra ngoài
  ngoài `println!`.

Hạn chế của thiết kế hiện tại:
- Polling 5 giây nghĩa là game có thể chạy được tối đa ~5 giây trước khi bị kill — không
  phải chặn ngay lập tức.
- Chỉ kill, không chặn việc mở lại: người dùng có thể mở lại game ngay sau khi bị kill,
  vòng lặp sẽ kill lại ở lần quét tiếp theo (tốn CPU/tiến trình lặp lại, trải nghiệm xấu,
  nhưng về chức năng vẫn chặn được, chỉ là có độ trễ).
- `src/main.rs` (được cargo build thật sự) hiện không chứa logic này — mới chỉ là
  `Hello, world!`. Đây là việc đầu tiên cần làm khi phát triển tiếp.

## Kiến trúc dự kiến (target)

Chia logic thành các module riêng trong `src/`:

- **`process_monitor`**: vòng lặp quét tiến trình, tái sử dụng ý tưởng từ
  `block-game.rs` nhưng tách thành struct/function độc lập, có thể test được, trả về
  sự kiện "phát hiện game X" thay vì tự xử lý luôn kill bên trong vòng lặp quét.
- **`blocker`**: nhận sự kiện phát hiện game, thực hiện chặn — kill tiến trình (giữ cách
  hiện tại), và hiện overlay chặn toàn màn hình (dùng crate GUI cho Windows, ví dụ
  `winit`/`windows-rs`, để vẽ cửa sổ full-screen luôn-on-top nhắc nhở người dùng).
- **`notifier`**: nhận sự kiện phát hiện/chặn game, gửi cảnh báo ra ngoài qua trait
  `Notifier` (một method, ví dụ `send(&self, event: &BlockEvent) -> Result<()>`). Có nhiều
  implementation, chọn 1 cái active tại một thời điểm — xem chi tiết ở mục
  [Thiết kế kênh thông báo](#thiết-kế-kênh-thông-báo) bên dưới.

**Không có module `config` đọc file bên ngoài** (quyết định: giữ chương trình đơn giản,
tải `.exe` về chạy luôn, không cần người dùng tự tạo/sửa file cấu hình). Danh sách game
cần chặn, webhook URL/bot token, loại kênh thông báo... đều là **hằng số khai báo thẳng
trong source** (ví dụ một file `src/settings.rs` với các `const`/`static`). Muốn đổi thì
sửa code rồi `cargo build --release` lại — chấp nhận đánh đổi này vì ứng dụng chỉ một
người dùng (chính tác giả) chỉnh sửa, không cần người dùng cuối tự cấu hình.

### Luồng xử lý

Vấn đề của thiết kế cũ: nếu vòng quét gọi thẳng `notifier.send()` và chờ HTTP response
(có thể mất vài trăm ms tới vài giây nếu mạng chậm/Discord rate-limit), thì chu kỳ quét
tiến trình bị kéo dài theo — tiến trình tiếp theo bị phát hiện trễ hơn. Thiết kế mới tách
hai việc **"phát hiện + chặn"** (ưu tiên nhanh, không được delay) ra khỏi **"gửi thông
báo"** (chịu được delay vài giây, có thể lỗi mạng) bằng một channel (`std::sync::mpsc`
hoặc `tokio::sync::mpsc` nếu dùng async runtime):

```
process_monitor (poll mỗi 1-2 giây, nhanh hơn 5s cũ vì refresh_processes rẻ)
        │
        ▼
  phát hiện game trong config.game_list, và game này CHƯA ở trạng thái "đang chặn"?
        │ (so điều kiện "chưa đang chặn" để không gửi lặp mỗi vòng quét
        │  khi user liên tục mở lại game — chỉ bắn sự kiện ở lần chuyển trạng thái)
        ▼
  blocker.block(process)        // kill ngay, đồng bộ, không chờ gì khác — đây là phần
        │                       // quan trọng nhất về độ trễ, giữ nguyên tốc độ như cũ
        ▼
  gửi BlockEvent vào channel (non-blocking, chỉ push vào queue)
        │
        ▼ (chạy ở task/thread riêng, độc lập với vòng quét)
  notifier worker: nhận event từ channel → notifier.send(event)
        │
        ├── thành công ──► xong
        └── lỗi (mất mạng, token sai...) ──► log lỗi, KHÔNG retry vô hạn (chỉ 1 kênh,
                                              không có fallback) — bỏ qua, chờ event tiếp theo
```

Nhờ tách ra, vòng quét `process_monitor` không bao giờ bị chậm lại vì lý do mạng/Discord,
và việc gửi thông báo chạy nền không chặn việc kill tiến trình kế tiếp.

### Theo dõi trạng thái (tránh spam)

`process_monitor` cần giữ một state nhỏ: tập hợp các game đang ở trạng thái "đã bị chặn,
chưa thấy thoát". Luồng xử lý:

- Lần đầu thấy game chạy → chuyển trạng thái `idle → blocked`, kill + gửi notify.
- Các lần quét sau, nếu game đó bị user mở lại và bị kill tiếp → **không gửi notify lại**
  (vẫn kill, nhưng im lặng), vì trạng thái đã là `blocked` rồi — tránh Discord bị spam
  tin nhắn mỗi 1-2 giây nếu user cứ mở lại game liên tục.
- Khi một vòng quét không còn thấy tiến trình đó nữa → chuyển `blocked → idle`, sẵn sàng
  gửi notify lại ở lần phát hiện tiếp theo.

## Thiết kế kênh thông báo

Cả Discord Webhook lẫn Discord Bot đều gắn với một **channel** cụ thể bên trong một
**server** (Discord không cho gửi tin "thẳng tới user" mà không qua channel/server nào).
Cách dùng: tạo một server Discord riêng tư (chỉ cho bản thân/gia đình, miễn phí, tạo trong
vài giây, không cần mời ai khác), tạo 1 channel text trong đó (vd `#canh-bao-game`), rồi
lấy Webhook URL hoặc invite Bot vào đúng channel đó.

`Notifier` là một trait chung, có 2 implementation cho MVP:

- **`DiscordWebhookNotifier`** (mặc định, dùng trước): lấy URL qua Channel Settings →
  Integrations → Webhooks → New Webhook → Copy URL. Gửi cảnh báo chỉ là một HTTP POST JSON
  `{"content": "..."}` tới URL đó — không cần giữ kết nối, không cần bot token. URL này
  chính là "credential" duy nhất, hard-code trong `src/settings.rs`, không commit lên git
  công khai nếu repo là public (xem ghi chú ở cuối mục này).
- **`DiscordBotNotifier`** (tùy chọn, implement sau nếu cần): tạo application + bot trong
  Discord Developer Portal, lấy bot token, invite bot vào server với quyền "Send Messages"
  trong channel đó, gửi bằng REST API `POST /channels/{channel_id}/messages` với header
  `Authorization: Bot <token>`. Phức tạp hơn webhook (phải quản lý token, giữ bot online
  nếu muốn nhận lệnh), nhưng mở đường cho tương tác 2 chiều sau này (vd gõ lệnh trong
  Discord để tạm mở chặn 10 phút).

Chọn implementation nào đang active cũng là một hằng số/lựa chọn trong code (không phải
config file), ví dụ comment/uncomment dòng khởi tạo `Notifier` trong `main.rs` khi build.

**Mở rộng sau này — gửi về server riêng (IP tự host)**: chỉ cần thêm một implementation
mới của cùng trait `Notifier`, ví dụ `HttpNotifier` gửi JSON tới `http://<ip>:<port>/alert`
— không đổi gì ở `process_monitor`/`blocker`, vì chúng chỉ biết tới interface `Notifier`,
không biết chi tiết Discord/HTTP bên trong.

**Lưu ý bảo mật**: vì webhook URL/bot token hard-code thẳng trong source, nếu repo Git là
public thì ai đọc được source cũng gửi tin nhắn giả vào kênh Discord đó được (URL/token
coi như "mật khẩu"). Nếu repo này là public, cân nhắc giữ file chứa secret ở nhánh riêng/
không commit, hoặc đổi webhook nếu lỡ lộ — không nằm trong phạm vi thiết kế này nhưng cần
lưu ý khi thực sự implement.

## Giả định & ràng buộc

- Chỉ chạy trên Windows (mục tiêu cuối: build `.exe`, có thể cross-compile từ Linux như
  mô tả trong `README.md`).
- Cần quyền đủ để kill tiến trình của chương trình khác — có thể yêu cầu chạy với quyền
  Administrator.
- Ứng dụng cá nhân/gia đình, ưu tiên **đơn giản**: tải `.exe` về chạy luôn, không có file
  cấu hình, không cần cài đặt gì thêm, không cần server/database ngoài — mọi tùy chỉnh
  (danh sách game, kênh thông báo) đều nằm trong source, đổi là phải build lại.

## Việc chưa quyết định (open question)

- **Cơ chế chặn việc mở lại game**: hiện tại chỉ dựa vào polling + kill lặp lại. Có cần
  cơ chế chặn sâu hơn (ví dụ chặn ở mức khởi động tiến trình, cần driver/hook hệ thống)
  hay không, vẫn chưa quyết định — việc này nằm ngoài phạm vi bản v0.1.0 và cần cân nhắc
  thêm về độ phức tạp/triển khai trước khi chọn hướng.
- **Cơ chế overlay chặn màn hình**: chưa chọn crate GUI cụ thể, cần khảo sát thêm khi bắt
  tay vào implement (ví dụ `winit` để tạo cửa sổ, hoặc gọi thẳng Win32 API qua
  `windows-rs` để kiểm soát always-on-top/full-screen tốt hơn).
- **Có implement `DiscordBotNotifier` ngay từ đầu hay chỉ làm `DiscordWebhookNotifier`
  trước**: webhook đã đủ cho mục tiêu "gửi cảnh báo 1 chiều", bot chỉ cần thiết nếu muốn
  tương tác 2 chiều — có thể hoãn bot lại, làm sau khi có nhu cầu thật.
