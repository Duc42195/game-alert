use std::collections::HashSet;
use std::sync::mpsc::Sender;
use std::thread;

use sysinfo::{ProcessExt, System, SystemExt};

use crate::blocker;
use crate::settings::{GAME_LIST, POLL_INTERVAL};

/// true nếu đây là lần đầu "game" chuyển từ idle sang blocked trong vòng quét này
/// (tức là chưa có trong tập các game đang bị chặn từ vòng quét trước) — chỉ ở lần
/// chuyển trạng thái này mới cần gửi thông báo, tránh spam mỗi vòng quét.
fn should_notify(game: &str, previously_blocked: &HashSet<String>) -> bool {
    !previously_blocked.contains(game)
}

/// Vòng lặp quét tiến trình vô hạn. Khi phát hiện + chặn một game lần đầu, gửi một
/// message cảnh báo (kèm `machine_label` để phân biệt máy nào khi chạy trên nhiều máy)
/// vào `notify_tx` (gửi vào channel là non-blocking, worker đọc ở thread riêng — xem
/// main.rs — nên vòng quét không bị chậm lại vì lý do mạng).
pub fn run(notify_tx: Sender<String>, machine_label: String) {
    let mut system = System::new_all();
    let mut blocked: HashSet<String> = HashSet::new();

    loop {
        system.refresh_processes();

        let mut still_running: HashSet<String> = HashSet::new();

        for &game in GAME_LIST {
            if let Some((_, process)) = system.processes().iter().find(|(_, p)| p.name() == game)
            {
                still_running.insert(game.to_string());

                if should_notify(game, &blocked) {
                    let msg = format!("[{machine_label}] Phát hiện và chặn tiến trình game: {game}");
                    if notify_tx.send(msg).is_err() {
                        crate::logging::log("Notifier worker đã dừng, không gửi được cảnh báo.");
                    }
                }

                blocker::block(process);
            }
        }

        blocked = still_running;

        thread::sleep(POLL_INTERVAL);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notifies_on_first_detection() {
        let blocked = HashSet::new();
        assert!(should_notify("RobloxPlayerBeta.exe", &blocked));
    }

    #[test]
    fn does_not_notify_again_while_still_blocked() {
        let mut blocked = HashSet::new();
        blocked.insert("RobloxPlayerBeta.exe".to_string());
        assert!(!should_notify("RobloxPlayerBeta.exe", &blocked));
    }

    #[test]
    fn notifies_again_after_returning_to_idle() {
        let game = "RobloxPlayerBeta.exe";
        let mut blocked = HashSet::new();

        // Vòng quét 1: lần đầu thấy game -> phải notify, rồi chuyển sang blocked.
        assert!(should_notify(game, &blocked));
        blocked.insert(game.to_string());

        // Vòng quét 2: game không còn chạy nữa -> blocked rỗng lại (như run() làm).
        blocked.clear();

        // Vòng quét 3: thấy lại game -> phải coi là lần đầu, notify lại.
        assert!(should_notify(game, &blocked));
    }
}
