# Provenance

Original Source: https://github.com/anreonyr/sqware.git
Frozen Source: `293c15fc97a9d9a4295810d10fe804e4cdec63c4`.

This first Router Unit repository snapshot contains the pure original API
prerequisite only. Router client/server/declaration and runtime integration remain
Source-owned and pending. The Driver Cluster will fix this Unit commit as a Git
submodule, and Hub uses its public API by a complete Git revision.

All five original API files below remain byte-identical, including the original
dependencies and macro surface. Only the standalone root workspace and repository
documentation are added. No fake runtime, UnitFile, test suite or ELF is supplied.
Native gates check actual metadata and host/RV API libraries; those are not a
claim of Router runtime, test-case coverage, or Kernel integration.

## Original source-history authors

- anreonyr <anreonyr@gmail.com>

The Source root has no project LICENSE. This snapshot invents none and does not rewrite Source or prior public histories. Existing file headers are retained.

## Original pre-adaptation file map

| Source path | Unit-owned path | Original SHA-256 |
| --- | --- | --- |
| `programs/src/driver/router/api/Cargo.toml` | `api/Cargo.toml` | `eb3e4c344473b5000246edf469d71b7d130a09bf80535902717cb038473d9379` |
| `programs/src/driver/router/api/src/frame.rs` | `api/src/frame.rs` | `c83b48f513a855ba9d85985d8efd4051f8429e4da6ae169e12af69c1d59957b4` |
| `programs/src/driver/router/api/src/lib.rs` | `api/src/lib.rs` | `857f873e88084552761167eddafe05ecf27cd30e93df3fdb76781750bf19c52b` |
| `programs/src/driver/router/api/src/marks.rs` | `api/src/marks.rs` | `29f7e70f52438f952c9483bc8a5126b4cdd6ce17925388b1a66fa6de102a048c` |
| `programs/src/driver/router/api/src/declaration.rs` | `api/src/declaration.rs` | `5d5795333f1a6a915cb0d2afaeb1991fef0e4ecace98010f550f2e2dc188abdb` |
