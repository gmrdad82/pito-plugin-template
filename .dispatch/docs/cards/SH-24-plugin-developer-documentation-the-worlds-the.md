# SH-24 — Plugin developer documentation: the worlds, the modules, themes, publishing

Wave: 1 · Run: single · Depends on: none · Complexity: **
Branch: split-sh-24 · Base: main

## What
`docs/` in pito-plugins, complete enough that a stranger with Rust and the `wasm32-wasip2` target builds, installs and runs a plugin on a desk from `docs/getting-started.md` alone: the worlds (with the WIT copied verbatim and each function explained), the manifest, the capabilities and what each grants, the slots, the UI tree and tones, the limits (fuel, deadline, memory, bounds) and what a fault looks like, the three module interfaces and the "needs the module" rule, `exec` and its rules, themes (`tokens.toml`, the contrast check, the picker), the registry (`index.json`, one-file release assets, the listing PR, what CI checks), versioning (`api`, `min_host_api`, semver), and a troubleshooting page. `templates/plugin` and `templates/theme` completed to build and validate on a clean machine. A `docs/README.md` index and the repo README pointing at it.

## How
1. **Copy the API, verbatim** (`wit/pito-host/0.1/host.wit`, `ui.wit`, `modules.wit`): from pito-tools at tag `v0.17.0` (`crates/plughost/wit/`), byte for byte; `wit/pito-host/0.1/SOURCE` records `pito-tools v0.17.0` and th
2. **`docs/getting-started.md`**: prerequisites (Rust stable, `rustup target add wasm32-wasip2`, `cargo add wit-bindgen`), copy `templates/plugin`, the `plugin.toml` fields one by one (SH-11's `Manifest`: `id` as `author/na
3. **`docs/worlds.md`**: `pito:host` interface by interface (`log`, `notify`, `kv`, `net`, `ui`, `page`), the world's exports (`info`, `activate`, `deactivate`, `on-event`, `render`), when each is called, what a plugin must
4. **`docs/capabilities.md`**: the list (`core:read`, `core:write`, `notify`, `net:<host>`, `mcp:tools`, `engine:use`, `remote:tunnel`, `exec:<program>`), what each unlocks, what the user sees at install (the grant sheet),
5. **`docs/ui-tree.md`**: the arena form (root and nodes, children as indices), every node and primitive with a JSON example, tones (the ten names, why no colours), events (`click`, `input`, `submit`, `key` and when `key` a
6. **`docs/limits.md`**: fuel per call, the deadline, the memory ceiling, one instance (copy the numbers from SH-11's `limits.rs` at the tag), what `Faulted` means and how a user re-enables, that a slow plugin degrades itse

## Guards
- Gate: the project's gate
- Accept: Every WIT file under `wit/pito-host/0.1/` is byte-identical to pito-tools `v0.17.0`'s (`sha256sum` in `SOURCE` matches) and validates.
- Accept: `docs/getting-started.md` was followed literally on a scratch home and produced a component (US-1).
- Accept: The template plugin installs, enables and renders on the Work desk from a local index (US-2, captures in the runbook).
