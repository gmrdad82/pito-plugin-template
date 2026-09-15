# pito-plugin-template

Start here to build a plugin or a theme for the PITO desks: Paper, Done
and PITO. A plugin is sandboxed WebAssembly (`wasm32-wasip2`) against the
`pito:host` world plus one world per desk; a theme is a palette-tokens
file and no code. A desk installs either from any public GitHub
repository by name: it reads the repository's latest release, downloads
the artifact and verifies its hash against the release's `SHA256SUMS`
before loading anything. There is no registry and nothing to submit.

Use this repository as a template ("Use this template" on GitHub), keep
the release workflow, write your plugin, tag `v1.0.0`. Repositories owned
by the desks' owner show an official mark in the Plugins screen; every
other repository installs the same way.

Under construction: the interface files, the templates and the docs land
here as the desks' plugin host takes shape.
