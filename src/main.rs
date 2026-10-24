mod blocker;
mod machine;
mod notifier;
mod process_monitor;
mod settings;

use std::sync::mpsc;
use std::thread;

use notifier::{DiscordWebhookNotifier, Notifier};

fn main() {
    let machine_label = machine::label();
    println!("Máy: {machine_label}");

    let (notify_tx, notify_rx) = mpsc::channel::<String>();

    // Worker riêng: chỉ việc này mới chịu độ trễ/lỗi mạng khi gửi Discord.
    let notifier_handle = thread::spawn(move || {
        let notifier = DiscordWebhookNotifier::new(settings::DISCORD_WEBHOOK_URL);
        for message in notify_rx {
            if let Err(e) = notifier.send(&message) {
                println!("Gửi thông báo thất bại: {e}");
            }
        }
    });

    // Vòng lặp chính: quét + chặn tiến trình, không bao giờ bị chặn bởi việc gửi Discord.
    process_monitor::run(notify_tx, machine_label);

    let _ = notifier_handle.join();
}
