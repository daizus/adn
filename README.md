# adn

**adn** is a declarative network orchestrator for Linux.

🚧 **v0.1.0 — Ephemeral only**  
Applies bridges, VLANs, and IPs at runtime using a `adn.toml` file.  
Persistent config generation (e.g., /etc/network/interfaces) is planned for later.

## Features

- Declarative TOML config
- Idempotent bridge and VLAN creation
- Static IP or DHCP support
- Preview, apply, list, and status commands
- Sensible defaults (`/etc/adn.toml`)

## Usage

```bash
sudo adn apply
adn preview [/path/to/config.toml]
adn status

