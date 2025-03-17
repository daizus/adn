# adn

**adn** is an Agnostic Declarative Network orchestrator for Linux.

⚡ `adn` applies network configurations like bridges, VLANs, static or dynamic IPs from a central TOML file — cleanly, idempotently, and fast.

---

## 🚧 v0.1.2 — Ephemeral runtime orchestrator

- Applies bridges, VLANs, namespaces and IPs at runtime (no reboot required)
- Includes rollback: auto-undo partial changes on failure
- Includes reset: fully remove bridges/VLANs from config
- Persistence to system-native config is coming in v0.2+

---

## 🔧 Features

- ✅ Declarative TOML config (`/etc/adn.toml` or custom path)
- ✅ Idempotent bridge & VLAN creation
- ✅ Static IP or DHCP support
- ✅ Commands:
  - `apply` – apply configuration
  - `preview` – dry-run (show what would be created)
  - `status` – show live interface state
  - `list` – show parsed config
  - `reset` – delete everything defined in config
- ✅ Transactional rollback on partial failure
- 🧱 Namespace + veth support (FreeBSD jail-style L2 isolation)

---

## 🔧 Example config

(default in */etc/adn.toml*)
```toml
[bridge.vmbr0]
interfaces = ['enp0s5']
dhcp = true

[bridge.InternalA]
interfaces = []
ip = "10.1.0.1/24"

[vlan.1043]
parent = 'vmbr0'
id = 1043
ip = '10.10.43.101/24'

[namespace.ns1]
veth = "vethA"
peer = "vethA-br"
ip = "10.1.0.2/24"
bridge = "InternalA"

[namespace.ns2]
veth = "vethB"
peer = "vethB-br"
ip = "10.1.0.3/24"
bridge = "InternalA"
```

---

## 🚀 Usage

```bash
# Apply the network config
sudo adn apply

# Preview changes
adn preview

# Show current system state
adn status

# Remove all bridges/VLANs defined in config
sudo adn reset

# Optional: specify config path
adn apply /path/to/custom.toml
```
