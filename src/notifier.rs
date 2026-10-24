/// Trait chung cho mọi kênh gửi cảnh báo. Thêm kênh mới (Discord Bot, HTTP tới server
/// riêng...) chỉ cần implement trait này, không đổi gì ở process_monitor/main.
pub trait Notifier: Send {
    fn send(&self, message: &str) -> Result<(), String>;
}

/// Gửi cảnh báo qua Discord Incoming Webhook (một chiều, không cần bot token).
pub struct DiscordWebhookNotifier {
    webhook_url: String,
}

impl DiscordWebhookNotifier {
    pub fn new(webhook_url: impl Into<String>) -> Self {
        Self {
            webhook_url: webhook_url.into(),
        }
    }
}

impl Notifier for DiscordWebhookNotifier {
    fn send(&self, message: &str) -> Result<(), String> {
        let body = format!("{{\"content\":\"{}\"}}", escape_json(message));
        ureq::post(&self.webhook_url)
            .set("Content-Type", "application/json")
            .send_string(&body)
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}

/// Escape tối thiểu để nhét một chuỗi text vào trong JSON string literal.
fn escape_json(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_quotes_and_backslashes() {
        assert_eq!(
            escape_json("hello \"world\"\\"),
            "hello \\\"world\\\"\\\\"
        );
    }

    #[test]
    fn escapes_newline_and_tab() {
        assert_eq!(escape_json("line1\nline2\tend"), "line1\\nline2\\tend");
    }

    #[test]
    fn leaves_plain_text_and_unicode_untouched() {
        assert_eq!(escape_json("Đã chặn Roblox"), "Đã chặn Roblox");
    }
}
