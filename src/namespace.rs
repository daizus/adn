use std::process::Command;

pub fn apply_namespace(
    name: &str,
    veth: &str,
    peer: &str,
    ip: &str,
    bridge: Option<&str>,
) -> Result<(), String> {
    println!("Creating namespace: {}", name);

    // Create the namespace
    Command::new("ip")
        .args(["netns", "add", name])
        .status()
        .map_err(|e| format!("Failed to create netns {}: {}", name, e))?;

    // Create veth pair
    Command::new("ip")
        .args(["link", "add", veth, "type", "veth", "peer", "name", peer])
        .status()
        .map_err(|e| format!("Failed to create veth pair: {}", e))?;

    // Move veth into namespace
    Command::new("ip")
        .args(["link", "set", veth, "netns", name])
        .status()
        .map_err(|e| format!("Failed to move veth into ns: {}", e))?;

    // Bring up peer side
    Command::new("ip")
        .args(["link", "set", peer, "up"])
        .status()
        .map_err(|e| format!("Failed to bring up peer: {}", e))?;

    // Inside namespace: assign IP, bring up veth and lo
    Command::new("ip")
        .args(["netns", "exec", name, "ip", "addr", "add", ip, "dev", veth])
        .status()
        .map_err(|e| format!("Failed to assign IP in ns: {}", e))?;

    Command::new("ip")
        .args(["netns", "exec", name, "ip", "link", "set", veth, "up"])
        .status()
        .ok();

    Command::new("ip")
        .args(["netns", "exec", name, "ip", "link", "set", "lo", "up"])
        .status()
        .ok();

    // Optionally attach peer to a bridge
    if let Some(bridge_name) = bridge {
        println!("Attaching {} to bridge {}", peer, bridge_name);
        Command::new("ip")
            .args(["link", "set", peer, "master", bridge_name])
            .status()
            .map_err(|e| format!("Failed to attach peer to bridge: {}", e))?;
    }

    Ok(())
}
