# This repository is public

It is read by strangers, and its laws follow from that.

- **Nothing internal, ever.** No path under a home directory, no hostname
  but github.com, no personal tooling, no tunnel, no client, no person,
  no agent name, no internal item number. The gate in
  `.github/workflows/validate.yml` lists only generic patterns.
- **A release is the plugin.** A `v*` tag builds the plugin and attaches
  `plugin.toml`, `plugin.wasm` and `SHA256SUMS` to the GitHub release. A
  desk installs from a repository by name and verifies the hash before
  loading. No registry, no listing, no submission.
- **Hashes are read, never typed.** `SHA256SUMS` is computed by CI from
  the bytes it built.
- **`wit/` is a copy.** The `pito:host` world and the per-desk worlds are
  owned upstream and mirrored here byte for byte; edit them upstream,
  never here.
- **Product names are the codenames** where the code needs one (work,
  pigeon, studio); the docs speak the shipped names (Paper, Done, PITO).

# The local gate

```
cargo fmt --check
cargo check --target wasm32-wasip2
actionlint
git diff --check
```

plus the two grep lines of `validate.yml`, run locally, both silent.

# Style

A short header comment per file saying what it owns. Plain prose, no
marketing. Security first.
