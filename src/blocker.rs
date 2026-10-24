use sysinfo::{Process, ProcessExt};

/// Kill một tiến trình đã phát hiện. Trả về true nếu kill thành công.
pub fn block(process: &Process) -> bool {
    let ok = process.kill();
    if ok {
        println!("Đã chặn tiến trình: {} (PID: {})", process.name(), process.pid());
    } else {
        println!(
            "Không thể chặn tiến trình: {} (PID: {})",
            process.name(),
            process.pid()
        );
    }
    ok
}
