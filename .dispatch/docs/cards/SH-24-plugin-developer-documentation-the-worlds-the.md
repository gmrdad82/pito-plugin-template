# SH-24 — Plugin developer documentation

Wave: 1 · Run: single · Depends on: none · Complexity: **
Branch: split-sh-24 · Base: main

## What
`docs/` tells a developer how to build, test and publish a plugin: the worlds, capabilities, the UI tree, limits; the WIT copied verbatim from the toolbox.

## How
1. Copy `wit/pito-host/0.1/*.wit` from pito-tools at the pinned tag.
2. `docs/getting-started.md`, `worlds.md`, `capabilities.md`, `ui-tree.md`, `limits.md`.
3. Follow getting-started on a scratch home; it produces a component.

## Guards
- Do not touch: `index.json`, `plugins/*`, `themes/*`.
- Gate: the project's gate
- Accept: every WIT file is byte-identical to the pinned tag's.
