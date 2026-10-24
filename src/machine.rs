use sysinfo::{System, SystemExt};

use crate::settings::REGISTERED_MACHINES;

/// Nhãn hiển thị cho máy hiện tại trong tin cảnh báo: tên thân thiện nếu máy đã có
/// trong `settings::REGISTERED_MACHINES`, hoặc machine id kèm "CHƯA ĐĂNG KÝ" nếu chưa.
pub fn label() -> String {
    label_for(&id(), REGISTERED_MACHINES)
}

fn label_for(machine_id: &str, registry: &[(&str, &str)]) -> String {
    match registry.iter().find(|(registered_id, _)| *registered_id == machine_id) {
        Some((_, name)) => name.to_string(),
        None => format!("máy CHƯA ĐĂNG KÝ (id: {machine_id})"),
    }
}

/// Id của máy hiện tại. Dùng hostname: đơn giản, không cần dependency mới (sysinfo đã
/// có sẵn). Hostname đổi nếu người dùng tự đổi tên máy — nếu sau này cần id sống sót
/// qua việc đổi tên, chuyển sang MachineGuid trong registry Windows (xem open-questions).
fn id() -> String {
    System::new().host_name().unwrap_or_else(|| "unknown-host".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registered_machine_shows_friendly_name() {
        let registry: &[(&str, &str)] = &[("DESKTOP-ABC123", "May phong khach")];
        assert_eq!(label_for("DESKTOP-ABC123", registry), "May phong khach");
    }

    #[test]
    fn unregistered_machine_shows_raw_id() {
        let registry: &[(&str, &str)] = &[("DESKTOP-ABC123", "May phong khach")];
        assert_eq!(
            label_for("DESKTOP-XYZ999", registry),
            "máy CHƯA ĐĂNG KÝ (id: DESKTOP-XYZ999)"
        );
    }

    #[test]
    fn empty_registry_always_shows_raw_id() {
        assert_eq!(label_for("anything", &[]), "máy CHƯA ĐĂNG KÝ (id: anything)");
    }
}
