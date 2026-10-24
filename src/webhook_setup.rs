use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::settings;

/// Discord webhook URL để dùng cho lần chạy này:
/// 1. Đã nhúng sẵn lúc build (qua build.rs từ secrets.env) -> dùng luôn, không hỏi gì.
/// 2. Chưa nhúng -> đọc từ `webhook.txt` cạnh file .exe (lưu từ lần chạy trước).
/// 3. Chưa có file đó -> hỏi qua console, lưu vào file đó cho những lần chạy sau.
pub fn resolve() -> String {
    if !settings::DISCORD_WEBHOOK_URL.is_empty() {
        return settings::DISCORD_WEBHOOK_URL.to_string();
    }

    let path = webhook_file_path();

    if let Ok(saved) = fs::read_to_string(&path) {
        let saved = saved.trim();
        if !saved.is_empty() {
            return saved.to_string();
        }
    }

    prompt_and_save(&path)
}

fn webhook_file_path() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."))
        .join("webhook.txt")
}

fn prompt_and_save(path: &PathBuf) -> String {
    loop {
        print!("Chưa có Discord webhook. Dán Discord Webhook URL rồi nhấn Enter: ");
        io::stdout().flush().ok();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("đọc input từ bàn phím");
        let url = input.trim();

        if url.is_empty() {
            println!("Webhook URL không được để trống, thử lại.");
            continue;
        }

        fs::write(path, url).expect("lưu webhook vào file cạnh .exe");
        println!("Đã lưu vào {} — lần sau sẽ không hỏi lại.", path.display());
        return url.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn webhook_file_sits_next_to_the_running_executable() {
        let path = webhook_file_path();
        assert_eq!(path.file_name().unwrap(), "webhook.txt");
        // current_exe() trong lúc `cargo test` là binary test, nhưng cơ chế lấy thư mục
        // cha là như nhau — chỉ cần xác nhận path có thư mục cha hợp lệ.
        assert!(path.parent().is_some());
    }
}
