# adn

**adn** is an Agnostic Declarative Network orchestrator for Linux.

⚡ `adn` applies network configurations like bridges, VLANs, static or dynamic IPs from a central TOML file — cleanly, idempotently, and fast.

---

## 🚧 v0.1.1 — Ephemeral runtime orchestrator

- Applies bridges, VLANs, and IPs at runtime (no reboot required)
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
