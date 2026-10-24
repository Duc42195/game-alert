use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

/// Ghi 1 dòng log ra console (nếu có) VÀ vào file `game-alert.log` cạnh file .exe. Bản
/// build với feature `background` không có console nào (`windows_subsystem = "windows"`)
/// nên `println!` không hiện ở đâu cả — file log là nơi duy nhất còn thấy được.
pub fn log(line: &str) {
    println!("{line}");

    let path = log_file_path();
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&path) {
        let _ = writeln!(file, "{line}");
    }
}

fn log_file_path() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."))
        .join("game-alert.log")
}
