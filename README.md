<!-- This page owns the template's first build and release instructions. -->
# pito-plugin-template

Start here to build a plugin for the PITO desks: Paper, Done and PITO. A
plugin is sandboxed WebAssembly (`wasm32-wasip2`) against the `pito:host`
world plus one world per desk. A desk installs one from any public GitHub
repository by name: it reads the repository's latest release, downloads
the artifact and verifies its hash against the release's `SHA256SUMS`
before loading anything. There is no registry and nothing to submit.

Use this repository as a template ("Use this template" on GitHub), keep
the release workflow, write your plugin, and tag a `v*` version. Official
repositories show a star in the Plugins screen; every other repository
installs the same way.

## Build

```sh
cargo build --release --target wasm32-wasip2
```

The placeholder artifact is the `.wasm` named after your crate under
`target/wasm32-wasip2/release/`. It keeps the template buildable until
the host's WIT worlds are mirrored into `wit/`.
