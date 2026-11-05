# Router Unit

Router provider slice: the frozen API/client prerequisites, the extracted
`router-server` runtime, and the repository-owned [`unit.toml`](unit.toml).
The current runtime policy uses `unit::document::Unit` schema 1 with stable id
and loader image `router`. It belongs to the complete Unit alongside its API
and server.

## Layout

- `api/` — canonical `router-api` at `94fec…`; the five files are byte-frozen.
- `client/` — original client at `aa3033a…`; the four files are byte-frozen.
- `server/` — `router-server` library and the RV64 `router` binary. Runtime bodies
  are the Source `c975cec…` bodies with only public-facade path normalization;
  `Cell` and `Reply` are extracted into one-structure files. The library exposes
  one public `serve()` entry (the old `main` body); `adapt`/`core`/`dev` are
  private and the bin is a thin `#[mold::entry]` caller.
- `unit.toml` — current Router runtime policy, compiled by the host to a binary
  record for System.
- `declaration/` — legacy `UNIT` and `BUILD`, retained for existing callers and
  migration comparisons; `BUILD` names `server/Cargo.toml` / package
  `router-server` / bin `router`.
- `tests/handoff/` — standalone host workspace with the four preserved handoff
  cases, projecting the real client and server helper files.
- `tests/declaration/` — standalone host workspace checking the real
  legacy `UnitFile`/`ImageBuild` field contract.
- `tests/unit-document/` — strict TOML/native field comparison, complete binary
  record roundtrip, full-width marks, and the real Cargo binary contract.

## Unit lifecycle and policy

`creation = "startup"` creates Router with the selected deployment. `requires`
selects the necessary Unit package closure; `after` waits for selected instances
to become ready. Both contain `hub`: Router finds Hub and claims its PLIC, device
tree and external-interrupt resources during startup. Inclusion through
`requires` does not start every included Unit: each declaration's `creation`
decides whether it starts with the deployment or waits for a creation request.
System must complete its Operator and Identity bootstrap before creating Units;
the old Operator bootstrap dependency therefore introduces no synthetic Unit id.

The service namespace remains shared at `router`, with subtree scope and
PART, UNPART, BIND, UNBIND and FIND authority. Its public `entry` publication
keeps `router_api::ENTRY_MARK`, STORE access and public permission. Identity
aliases and grants, supply bindings, construction and retirement remain empty;
supervision and source mark remain absent. Readiness uses the same
`unit::READY_MARK` / `router_api::READY_MARK` signal, and ending stays resident.
The new groups list is empty and guarded is false. Marks and authority bits are
checked against actual Rust provider constants. Full-width `u64` marks use
quoted decimals; schema 1 also accepts quoted `0x` hexadecimal and keeps native
`u64` values in binary records.

The host discovers the actual Cargo binary in `server/Cargo.toml`: package
`router-server`, bin/image `router`, entry `server/src/main.rs`, target
`riscv64gc-unknown-none-elf`, release profile, default features enabled and no
extra features. Build metadata is separate from runtime policy. A Cluster only
organizes Unit references; it is not a runtime Unit. The host assembles each
selected deployment in memory from declarations and their dependencies, with no
new deployment file or Unit registry in this repository.

## Public identities

| Component | Repository | Revision |
| --- | --- | --- |
| `router-api` | `https://github.com/shuyra-arche/router.git` | `94fec223f3c5df6c99a49527a65d016d88de3af4` |
| `router-client` (via `driver-shared`) | `https://github.com/shuyra-arche/router.git` | `aa3033a7bd25b5d4199a50a6c00538475b62ef4c` |
| `driver-shared` | `https://github.com/shuyra-arche/driver.git` | `e75b1e232f828666956fb7e02c61a7d84f13eb66` |
| `env` | `https://github.com/shuyra-arche/env.git` | `d3000253cd5d13d466e91fafd4dbb27a5ec5aaae` |
| `wire` | `https://github.com/shuyra-arche/wire.git` | `0ff77dc4573fa2620d2fe995a18c01949d9c7e57` |
| `resource` | `https://github.com/shuyra-arche/resource.git` | `dbc96834f50f094b5c386e54a0a2ae5560dcd7f8` |
| `ipc` | `https://github.com/shuyra-arche/ipc.git` | `29ad97dc08d9dfc4ea6578b1b77874261229b3b5` |
| `system-api` | `https://github.com/shuyra-arche/system.git` | `9e5d63ca536a176583cf1b0054b4ecf7ddd88257` |
| `system-client`/`system-support` | `https://github.com/shuyra-arche/system.git` | `5a792ff2d39901a970925b1a6d63206adf6c16d8` |
| `hub-api` | `https://github.com/shuyra-arche/hub.git` | `cccef3f325198c8390007ae88b064bae0f21f93e` |
| `unit` (production legacy) | `https://github.com/shuyra-arche/unit.git` | `90396f0fda376e0693fb1b9553aa21864d241057` |
| `unit` (document tests) | `https://github.com/shuyra-arche/unit.git` | `ad50b803c3bd5cd2ef90010d80badf23551e88d5` |
| `execution` | `https://github.com/shuyra-arche/execution.git` | `1b333a28bc4c34beaea3f7986a9bb6e8ab2db414` |
| `mold` | `https://github.com/shuyra-arche/mold.git` | `f52501f4c91b686eded8b4bcd743b4e96084b7f9` |

The server has no direct `router-client` dependency and no local `api`/`client`
path identity; the frozen local members coexist for source projection only.

## Root-owned gates

```sh
cargo test --locked --manifest-path tests/handoff/Cargo.toml --target x86_64-unknown-linux-gnu
cargo test --locked --manifest-path tests/declaration/Cargo.toml --target x86_64-unknown-linux-gnu
cargo test --locked --manifest-path tests/unit-document/Cargo.toml --target x86_64-unknown-linux-gnu
export CARGO_TARGET_RISCV64GC_UNKNOWN_NONE_ELF_RUSTFLAGS='-Crelocation-model=static -Cforce-frame-pointers=yes -Ccode-model=medium'
cargo build --locked --target riscv64gc-unknown-none-elf -p router-server --bin router
cargo build --locked --release --target riscv64gc-unknown-none-elf -p router-server --bin router
```

The document suite pins the public Unit document/record API at
`ad50b803c3bd5cd2ef90010d80badf23551e88d5` and owns its committed Cargo lock.
Its `legacy-unit` test alias keeps the old enum types for comparison with
`router-declaration`. Production manifests and their existing locks retain
`90396f0fda376e0693fb1b9553aa21864d241057`. The document checks use the normal
manifest directly, with no path patch or staged manifest:

```sh
source /workspace/.vanllia-tools/env.sh
flock /tmp/router-unit-public-target.lock cargo test --locked \
  --manifest-path tests/unit-document/Cargo.toml \
  --target-dir /tmp/router-unit-public-target --target x86_64-unknown-linux-gnu
```

Once dependencies are cached, the same command also accepts `--offline`. These
host suites prove handoff helpers, declaration policy and binary encoding. PLIC/IRQ,
Kernel, System, hardware and real-image acceptance remain separate work. Source
intake and the existing test scenarios are unchanged.

See `PROVENANCE.md` for the frozen Source anchor and the preserved-file policy.
Root's actual checks and limitations are recorded in `VALIDATION.md`.
