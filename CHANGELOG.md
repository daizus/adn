# Changelog

## v0.1.2 - Namespace support

### Added
- ✨ Declarative network namespaces with veth pairs
- 🧠 `adn status` now inspects and reports interfaces inside namespaces
- 📜 `adn list` now includes namespace definitions
- 🧼 `adn reset` removes namespaces and associated veth interfaces
- ✅ `preview` includes planned namespace creation

### Fixed
- `status` properly detects missing namespaces and veths
- Improved output consistency and error detection

---

## v0.1.1 - Transactional rollback & reset

### Added
- `rollback` support: interfaces/IPs/DHCP are automatically undone if apply fails
- `reset` command: removes all bridges and VLANs defined in the config
- Enum-based `AppliedChange` tracking (with action-aware rollback)

### Improved
- Error reporting for VLAN/bridge failures
- Interface existence checks before delete
- `status` shows `MISSING` when interfaces are gone
- Silenced unused-variable warnings with `_changes` convention

---

## v0.1.0 - Initial release

- Declarative network config via `adn.toml`
- Support for bridges (static & DHCP)
- VLAN creation and IP assignment
- Commands: `apply`, `preview`, `status`, `list`
- Default config: `/etc/adn.toml`
- Ephemeral-only configuration (runtime-only)

