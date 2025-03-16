# netcraft

**netcraft** is a declarative network orchestrator for Linux.

🚧 **v0.1.0 — Ephemeral only**  
Applies bridges, VLANs, and IPs at runtime using a `netcraft.toml` file.  
Persistent config generation (e.g., /etc/network/interfaces) is planned for later.

## Features

- Declarative TOML config
- Idempotent bridge and VLAN creation
- Static IP or DHCP support
- Preview, apply, list, and status commands
- Sensible defaults (`/etc/netcraft.toml`)

## Usage

```bash
sudo netcraft apply
netcraft preview [/path/to/config.toml]
netcraft status

