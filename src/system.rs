use std::process::Command;

/// Check if a network interface already exists
pub fn interface_exists(name: &str) -> bool {
    let output = Command::new("ip").args(["link", "show", name]).output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Check if an interface already has an IPv4 address
pub fn has_ip(name: &str) -> bool {
    let output = Command::new("ip")
        .args(["-4", "addr", "show", name])
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout
                .lines()
                .any(|line| line.trim_start().starts_with("inet "))
        }
        Err(_) => false,
    }
}
