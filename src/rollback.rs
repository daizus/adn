use crate::types::{AppliedChange, ApplyAction};
use std::process::Command;

pub fn rollback(changes: &[AppliedChange]) {
    for change in changes.iter().rev() {
        match &change.action {
            ApplyAction::AssignedStaticIp(ip) => {
                eprintln!("⚠️  Rolling back IP {} on {}", ip, change.iface);
                let _ = Command::new("ip")
                    .args(["addr", "del", ip, "dev", &change.iface])
                    .status();
            }

            ApplyAction::RanDhcp => {
                eprintln!("⚠️  Releasing DHCP on {}", change.iface);
                let _ = Command::new("dhclient")
                    .args(["-r", &change.iface])
                    .status();
            }

            ApplyAction::CreatedInterface => {
                eprintln!("⚠️  Deleting interface {}", change.iface);
                let _ = Command::new("ip")
                    .args(["link", "delete", &change.iface])
                    .status();
            }
        }
    }
}
