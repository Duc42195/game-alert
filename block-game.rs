use std::{thread, time::Duration};
use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    // Danh sách các game cần kiểm tra
    let game_list = vec!["RobloxPlayerBeta.exe"];

    // Tạo một đối tượng System để lưu trạng thái hệ thống
    let mut system = System::new_all();

    loop {
        // Làm mới thông tin tiến trình (chỉ tiến trình, không làm mới toàn bộ)
        system.refresh_processes();

        // Duyệt qua danh sách các tiến trình cần kiểm tra
        for game in &game_list {
            // Kiểm tra nếu tiến trình đang chạy
            if let Some((pid, process)) = system.processes().iter().find(|(_, p)| p.name() == *game) {
                println!("Đã phát hiện tiến trình: {} (PID: {})", process.name(), pid);

                // Kill tiến trình
                if let Err(e) = process.kill() {
                    println!("Không thể dừng tiến trình {}: {:?}", process.name(), e);
                } else {
                    println!("Đã dừng tiến trình: {}", process.name());
                }
            }
        }

        // Chờ 5 giây trước khi kiểm tra lại
        thread::sleep(Duration::from_secs(5));
    }
}