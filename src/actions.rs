use crate::config::load_config;
use crate::system::{has_ip, interface_exists, interface_has_ip};
use crate::types::{AppliedChange, ApplyAction};
use crate::{namespace, rollback};
use std::path::Path;
use std::process::Command;

pub fn list(config_path: &Path) {
    let config = load_config(config_path);

    println!("Bridges:");
    if let Some(bridges) = config.bridge {
        for (name, bridge) in bridges {
            let iface_list = bridge.interfaces.join(", ");
            let dhcp = bridge.dhcp.unwrap_or(false);
            let ip = bridge.ip.unwrap_or_else(|| "-".into());
            println!(
                "  - {} (interfaces: {}, ip: {}, dhcp: {})",
                name, iface_list, ip, dhcp
            );
        }
    } else {
        println!("  (none)");
    }

    println!("\nVLANs:");
    if let Some(vlans) = config.vlan {
        for (name, vlan) in vlans {
            let ip = vlan.ip.as_deref().unwrap_or("-");
            println!(
                "  - {} (id: {}, parent: {}, ip: {})",
                name, vlan.id, vlan.parent, ip
            );
        }
    } else {
        println!("  (none)");
    }

    if let Some(namespaces) = config.namespace {
        println!("\nNamespaces:");
        for (name, ns) in namespaces {
            println!(
                "  - {} (veth: {} <-> {}, ip: {})",
                name, ns.veth, ns.peer, ns.ip
            );
            if let Some(br) = ns.bridge {
                println!("    Attached to bridge: {}", br);
            }
        }
    }
}

pub fn status(config_path: &Path) {
    let config = load_config(config_path);

    println!("Status:");

    if let Some(bridges) = config.bridge {
        for (name, _) in bridges {
            print_interface_status(&name);
        }
    }

    if let Some(vlans) = config.vlan {
        for (name, _) in vlans {
            print_interface_status(&name);
        }
    }

    if let Some(namespaces) = config.namespace {
        println!("\nNamespaces:");

        for (name, ns) in namespaces {
            // Build: ip netns exec <name> ip -4 addr show <veth>
            let output = Command::new("ip")
                .args(["netns", "exec", &name, "ip", "-4", "addr", "show", &ns.veth])
                .output();

            match output {
                Ok(output) if output.status.success() => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let mut iface_ip = "-".to_string();
                    let mut iface_state = "DOWN".to_string();

                    for line in stdout.lines() {
                        if line.trim_start().starts_with("inet ") {
                            let parts: Vec<&str> = line.split_whitespace().collect();
                            if parts.len() >= 2 {
                                iface_ip = parts[1].to_string();
                            }
                        }

                        if line.trim_start().starts_with("state") {
                            let parts: Vec<&str> = line.split_whitespace().collect();
                            if parts.len() >= 2 {
                                iface_state = parts[1].to_string();
                            }
                        }
                    }

                    println!(
                        "  [{:<6}] {:<10} {:<6} {}",
                        name, ns.veth, iface_state, iface_ip
                    );
                }

                _ => {
                    println!("  [{:<6}] {:<10} MISSING", name, ns.veth);
                }
            }
        }
    }
}

fn print_interface_status(name: &str) {
    if !interface_exists(name) {
        println!("  {:<12} MISSING", name);
        return;
    }

    // Get interface state
    let state = get_interface_state(name).unwrap_or("UNKNOWN".into());

    // Get IP address
    let ip = get_interface_ip(name).unwrap_or_else(|| "-".to_string());

    println!("  {:<12} {:<6} {}", name, state, ip);
}

fn get_interface_state(name: &str) -> Option<String> {
    let output = Command::new("ip")
        .args(["link", "show", name])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains(name) {
            if line.contains("state UP") {
                return Some("UP".to_string());
            } else if line.contains("state DOWN") {
                return Some("DOWN".to_string());
            }
        }
    }

    None
}

fn get_interface_ip(name: &str) -> Option<String> {
    let output = Command::new("ip")
        .args(["-4", "addr", "show", name])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.trim_start().starts_with("inet ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                return Some(parts[1].to_string()); // e.g. "192.168.10.1/24"
            }
        }
    }

    None
}

pub fn preview(config_path: &Path) {
    let config = load_config(config_path);
    println!("--- Previewing config ---");
    if let Some(bridges) = config.bridge {
        for (name, bridge) in bridges {
            println!("Would create bridge: {name}");
            println!("  Interfaces: {:?}", bridge.interfaces);
            if let Some(ip) = bridge.ip {
                println!("  Assign IP: {ip}");
            }
            if bridge.dhcp.unwrap_or(false) {
                println!("  DHCP enabled");
            }
        }
    }

    if let Some(vlans) = config.vlan {
        for (name, vlan) in vlans {
            println!("Would create VLAN: {name} (id {})", vlan.id);
            println!("  Parent interface: {}", vlan.parent);
            if let Some(ip) = vlan.ip {
                println!("  Assign IP: {ip}");
            }
        }
    }

    if let Some(namespaces) = config.namespace {
        for (name, ns) in namespaces {
            println!("Would create namespace: {}", name);
            println!("  veth pair: {} <-> {}", ns.veth, ns.peer);
            println!("  Assign IP: {}", ns.ip);
            if let Some(br) = &ns.bridge {
                println!("  Attach to bridge: {}", br);
            }
        }
    }
}

pub fn apply(config_path: &Path) {
    let mut changes = Vec::new();

    if let Err(e) = try_apply(config_path, &mut changes) {
        eprintln!("❌ Apply failed: {}", e);
        rollback::rollback(&changes);
    }
}

fn try_apply(config_path: &Path, _changes: &mut Vec<AppliedChange>) -> Result<(), String> {
    let config = load_config(config_path);
    let mut changes: Vec<AppliedChange> = Vec::new();

    if let Some(bridges) = config.bridge {
        for (name, bridge) in bridges {
            println!("Applying bridge: {}", name);

            // 1. Create bridge if it doesn't already exist
            if !interface_exists(&name) {
                println!("Checking bridge: {}", name);
                let status = Command::new("ip")
                    .args(["link", "add", &name, "type", "bridge"])
                    .status()
                    .map_err(|e| format!("Failed to create bridge {}: {}", name, e))?;

                if !status.success() {
                    return Err(format!("Bridge creation failed: {}", name));
                }

                changes.push(AppliedChange {
                    iface: name.clone(),
                    action: ApplyAction::CreatedInterface,
                });
            } else {
                println!("Bridge {} already exists. Skiping creation.", name);
            }

            // 2. Add interfaces to bridge
            for iface in &bridge.interfaces {
                let _ = Command::new("ip")
                    .args(["link", "set", iface, "master", &name])
                    .status()
                    .expect("Failed to add interface to bridge.");
            }

            // 3. Bring up the bridge
            let _ = Command::new("ip")
                .args(["link", "set", &name, "up"])
                .status()
                .expect("Failed to bring up bridge.");

            // 4. Bring up each interface
            for iface in &bridge.interfaces {
                let _ = Command::new("ip")
                    .args(["link", "set", iface, "up"])
                    .status()
                    .expect("Failed to bring up interface");
            }

            // 5. Assign static IP if defined
            if let Some(ip) = &bridge.ip {
                println!("Assigning static IP {} to {}", ip, name);
                let _ = Command::new("ip")
                    .args(["addr", "add", ip, "dev", &name])
                    .status()
                    .expect("Failed to assign IP address");
                changes.push(AppliedChange {
                    iface: name.clone(),
                    action: ApplyAction::AssignedStaticIp(ip.clone()),
                });
            } else if bridge.dhcp.unwrap_or(false) {
                if has_ip(&name) {
                    println!("{} already has an IP address. Skipping dhclient.", name);
                } else {
                    println!("Running dhclient on {}", name);
                    let _ = Command::new("dhclient")
                        .arg(&name)
                        .status()
                        .expect("Failed to run dhclient");
                    changes.push(AppliedChange {
                        iface: name.clone(),
                        action: ApplyAction::RanDhcp,
                    });
                }
            }
        }
        println!("Done.");
    }

    if let Some(vlans) = config.vlan {
        for (name, vlan) in vlans {
            println!("Applying VLAN: {} (id {})", name, vlan.id);

            if !interface_exists(&name) {
                let status = Command::new("ip")
                    .args([
                        "link",
                        "add",
                        "link",
                        &vlan.parent,
                        "name",
                        &name,
                        "type",
                        "vlan",
                        "id",
                        &vlan.id.to_string(),
                    ])
                    .status()
                    .map_err(|e| format!("Failed to create VLAN {}: {}", name, e))?;

                if !status.success() {
                    return Err(format!("VLAN creation failed: {}", name));
                }

                changes.push(AppliedChange {
                    iface: name.clone(),
                    action: ApplyAction::CreatedInterface,
                });
            } else {
                println!("VLAN interface {} already exists. Skipping creation.", name);
            }

            if let Some(ip) = &vlan.ip {
                if interface_has_ip(&name, ip) {
                    println!("{} already has IP {}. Skipping.", name, ip);
                } else {
                    println!("Assigning IP {}: to {}", ip, name);
                    let _ = Command::new("ip")
                        .args(["addr", "add", ip, "dev", &name])
                        .status()
                        .expect("Failed to assign IP to VLAN interface");
                    changes.push(AppliedChange {
                        iface: name.clone(),
                        action: ApplyAction::AssignedStaticIp(ip.clone()),
                    });
                }
            }

            let _ = Command::new("ip")
                .args(["link", "set", &name, "up"])
                .status()
                .expect("Failed to bring up VLAN interface.");
        }
    }

    if let Some(namespaces) = config.namespace {
        for (name, ns) in namespaces {
            namespace::apply_namespace(&name, &ns.veth, &ns.peer, &ns.ip, ns.bridge.as_deref())
                .map_err(|e| format!("Namespace {} setup failed: {}", name, e))?;
        }
    }

    Ok(())
}

pub fn reset(config_path: &Path) {
    let config = load_config(config_path);

    if let Some(vlans) = config.vlan {
        for (name, _) in vlans {
            if interface_exists(&name) {
                println!("Deleting VLAN interface: {}", name);
                let _ = Command::new("ip").args(["link", "delete", &name]).status();
            } else {
                println!("VLAN {} not found, skipping", name);
            }
        }
    }

    if let Some(bridges) = config.bridge {
        for (name, _) in bridges {
            if interface_exists(&name) {
                println!("Deleting bridge interface: {}", name);
                let _ = Command::new("ip")
                    .args(["link", "delete", &name, "type", "bridge"])
                    .status();
            } else {
                println!("Bridge {} not found, skipping", name);
            }
        }
    }

    if let Some(namespaces) = config.namespace {
        for (name, ns) in namespaces {
            // Delete the veth peer (on the host)
            if interface_exists(&ns.peer) {
                println!("Deleting veth peer interface: {}", ns.peer);
                let _ = Command::new("ip")
                    .args(["link", "delete", &ns.peer])
                    .status();
            } else {
                println!("veth peer {} not found, skipping", ns.peer);
            }

            // Delete the namespace
            println!("Deleting namespace: {}", name);
            let _ = Command::new("ip").args(["netns", "delete", &name]).status();
        }
    }

    println!("✅ Reset complete.");
}
