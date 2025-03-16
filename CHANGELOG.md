# Changelog

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

