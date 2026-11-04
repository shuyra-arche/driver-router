# Provenance

Original Source: https://github.com/anreonyr/sqware.git
Frozen Source: `293c15fc97a9d9a4295810d10fe804e4cdec63c4`.
Client source anchor: `ac5b61d8880c499f8cee4c60929c06cc9f4820d8`.

This Router Unit repository snapshot contains the original API prerequisite and
the original client. Router server, declaration, shared Context/Device/Fail
support and runtime integration remain Source-owned and pending. The Driver
Cluster fixes this Unit commit as a Git submodule, and Hub uses only its public
API by a complete Git revision. The client is a prerequisite for future explicit
shared Context/Device/Fail support and Router server extraction, not a Hub
dependency.

All five original API files and all four original client files below remain
byte-identical, including the original dependencies, macro surface, packet
shapes and public names. The client keeps the canonical `router-api` Git pin
`94fec223f3c5df6c99a49527a65d016d88de3af4` and has no `path` dependency on the
local API. Only the standalone root workspace, the client workspace membership
and repository documentation are added. No fake runtime, UnitFile, test suite or
ELF is supplied. Native gates check actual metadata and host/RV API libraries;
those are not a claim of Router runtime, test-case coverage, or Kernel
integration.

## Original source-history authors

- anreonyr <anreonyr@gmail.com>

The Source root has no project LICENSE. This snapshot
invents none and does not rewrite Source or prior public histories. Existing file headers are retained.

## Original pre-adaptation file map

| Source path | Unit-owned path | Original SHA-256 |
| --- | --- | --- |
| `programs/src/driver/router/api/Cargo.toml` | `api/Cargo.toml` | `eb3e4c344473b5000246edf469d71b7d130a09bf80535902717cb038473d9379` |
| `programs/src/driver/router/api/src/frame.rs` | `api/src/frame.rs` | `c83b48f513a855ba9d85985d8efd4051f8429e4da6ae169e12af69c1d59957b4` |
| `programs/src/driver/router/api/src/lib.rs` | `api/src/lib.rs` | `857f873e88084552761167eddafe05ecf27cd30e93df3fdb76781750bf19c52b` |
| `programs/src/driver/router/api/src/marks.rs` | `api/src/marks.rs` | `29f7e70f52438f952c9483bc8a5126b4cdd6ce17925388b1a66fa6de102a048c` |
| `programs/src/driver/router/api/src/declaration.rs` | `api/src/declaration.rs` | `5d5795333f1a6a915cb0d2afaeb1991fef0e4ecace98010f550f2e2dc188abdb` |

## Original pre-adaptation client file map

The four client files were copied byte-identically from Source
`ac5b61d8880c499f8cee4c60929c06cc9f4820d8`; their SHA-256 values are recorded
from the Root provider gate.

| Source path | Unit-owned path | Original SHA-256 |
| --- | --- | --- |
| `programs/src/driver/router/client/Cargo.toml` | `client/Cargo.toml` | `a04887bd766d45d4e8d44aa553b95ffe911d51701b7a331fa05d1487419016a0` |
| `programs/src/driver/router/client/src/lib.rs` | `client/src/lib.rs` | `385cf8f0824a03f7ac371222c031f650b1e00fd227ddd9099ec5a534a691978a` |
| `programs/src/driver/router/client/src/line.rs` | `client/src/line.rs` | `f135417c8d57d90591da36a911e338813cd69a457e8bd11d8dc560d2475338a7` |
| `programs/src/driver/router/client/src/handoff.rs` | `client/src/handoff.rs` | `710e7b089fe277988b8933f592d104c515f94c6d6f89df22d86bdbf9e4289147` |
