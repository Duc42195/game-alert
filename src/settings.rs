use std::time::Duration;

/// Danh sách tên tiến trình (.exe) cần chặn, so khớp chính xác (case-sensitive).
#[cfg(not(feature = "smoke-test"))]
pub const GAME_LIST: &[&str] = &["RobloxPlayerBeta.exe"];

/// Bản dùng để tự test (`cargo run --features smoke-test`): thêm "game-alert-test" —
/// chương trình giả trong `scripts/`, không làm gì cả, chỉ để kill thử. Tên này KHÔNG
/// phải "powershell.exe"/"bash" vì chạy thẳng interpreter sẽ match nhầm mọi tiến trình
/// PowerShell/bash khác đang mở — xem scripts/README.md cách chạy đúng.
#[cfg(feature = "smoke-test")]
pub const GAME_LIST: &[&str] = &["RobloxPlayerBeta.exe", "game-alert-test.exe", "game-alert-test"];

/// Chu kỳ quét lại danh sách tiến trình.
pub const POLL_INTERVAL: Duration = Duration::from_secs(2);

/// Máy đã đăng ký: (machine id = hostname, tên thân thiện hiển thị trong cảnh báo).
/// Máy KHÔNG có trong danh sách này vẫn hoạt động bình thường — tin cảnh báo sẽ hiện
/// thẳng hostname kèm "CHƯA ĐĂNG KÝ" thay vì tên thân thiện. Cách đăng ký máy mới: chạy
/// thử 1 lần, xem hostname hiện ra ở dòng "Máy: ..." lúc khởi động hoặc trong tin Discord
/// đầu tiên, copy vào đây, build lại.
pub const REGISTERED_MACHINES: &[(&str, &str)] = &[
    ("Ducky", "Ducky-Ubuntu"),
    // ("TEN-MAY-WINDOWS", "Tên dễ hiểu, vd Máy phòng khách"),
];

/// Discord webhook URL nhúng lúc compile bởi build.rs từ `.agents/state/secrets.env`
/// (ưu tiên) hoặc `~/secret.env` (fallback) — NẾU có. Cho phép rỗng: bản build công khai
/// (release tải từ GitHub, build không kèm secrets.env nào) sẽ rỗng ở đây một cách hợp lệ
/// — `src/webhook_setup.rs` sẽ hỏi người dùng nhập lúc chạy lần đầu và lưu lại, không bao
/// giờ hard-code trực tiếp giá trị thật vào đây.
pub const DISCORD_WEBHOOK_URL: &str = match option_env!("DISCORD_WEBHOOK_URL") {
    Some(url) => url,
    None => "",
};
