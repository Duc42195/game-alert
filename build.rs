// Nạp secret (vd DISCORD_WEBHOOK_URL) vào biến môi trường của quá trình compile, để
// src/settings.rs dùng option_env!() nhúng thẳng vào binary — secret không bao giờ nằm
// trong file .rs nào, không vào git. Ưu tiên secrets riêng của project, nếu không có thì
// lấy ở secrets chung tại $HOME/secret.env. Không tìm thấy thì bỏ qua (không lỗi) — binary
// build ra sẽ có DISCORD_WEBHOOK_URL rỗng, và src/webhook_setup.rs sẽ hỏi người dùng nhập
// lúc chạy lần đầu (dùng cho bản build công khai, không kèm secrets.env nào, vd release).
use std::env;
use std::fs;
use std::path::PathBuf;

const KEYS_NEEDED: &[&str] = &["DISCORD_WEBHOOK_URL"];

fn main() {
    let project_secrets = PathBuf::from(".agents/state/secrets.env");
    println!("cargo:rerun-if-changed={}", project_secrets.display());

    let home_secrets = env::var_os("HOME").map(|h| PathBuf::from(h).join("secret.env"));
    if let Some(p) = &home_secrets {
        println!("cargo:rerun-if-changed={}", p.display());
    }

    for key in KEYS_NEEDED {
        let value = [Some(project_secrets.clone()), home_secrets.clone()]
            .into_iter()
            .flatten()
            .find_map(|path| fs::read_to_string(&path).ok().and_then(|c| find_key(&c, key)));

        if let Some(value) = value {
            println!("cargo:rustc-env={key}={value}");
        }
    }
}

fn find_key(content: &str, key: &str) -> Option<String> {
    content.lines().find_map(|line| {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return None;
        }
        let (k, v) = line.split_once('=')?;
        (k.trim() == key).then(|| v.trim().to_string())
    })
}
