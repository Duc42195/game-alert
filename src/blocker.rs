use sysinfo::{Process, ProcessExt};

use crate::logging;

/// Kill một tiến trình đã phát hiện. Trả về true nếu kill thành công.
pub fn block(process: &Process) -> bool {
    let ok = process.kill();
    if ok {
        logging::log(&format!(
            "Đã chặn tiến trình: {} (PID: {})",
            process.name(),
            process.pid()
        ));
    } else {
        logging::log(&format!(
            "Không thể chặn tiến trình: {} (PID: {})",
            process.name(),
            process.pid()
        ));
    }
    ok
}
