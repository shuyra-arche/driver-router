# Router Unit

Router provider slice: the frozen API/client prerequisites, the extracted
`router-server` runtime, and the Router `UNIT`/`BUILD` declaration.

## Layout

- `api/` — canonical `router-api` at `94fec…`; the five files are byte-frozen.
- `client/` — original client at `aa3033a…`; the four files are byte-frozen.
- `server/` — `router-server` library and the RV64 `router` binary. Runtime bodies
  are the Source `c975cec…` bodies with only public-facade path normalization;
  `Cell` and `Reply` are extracted into one-structure files. The library exposes
  one public `serve()` entry (the old `main` body); `adapt`/`core`/`dev` are
  private and the bin is a thin `#[mold::entry]` caller.
- `declaration/` — the only Router `UnitFile` owner: `UNIT` (namespace, binding,
  `Operator`+`hub` order, ready signal, resident ending) and `BUILD` for
  `server/Cargo.toml` / package `router-server` / bin `router`.
- `tests/handoff/` — standalone host workspace with the four preserved handoff
  cases, projecting the real client and server helper files.
- `tests/declaration/` — standalone host workspace checking the real
  `UnitFile`/`ImageBuild` field contract.

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
| `unit` | `https://github.com/shuyra-arche/unit.git` | `90396f0fda376e0693fb1b9553aa21864d241057` |
| `execution` | `https://github.com/shuyra-arche/execution.git` | `1b333a28bc4c34beaea3f7986a9bb6e8ab2db414` |
| `mold` | `https://github.com/shuyra-arche/mold.git` | `f52501f4c91b686eded8b4bcd743b4e96084b7f9` |

The server has no direct `router-client` dependency and no local `api`/`client`
path identity; the frozen local members coexist for source projection only.

## Root-owned gates

```sh
cargo test --locked --manifest-path tests/handoff/Cargo.toml --target x86_64-unknown-linux-gnu
cargo test --locked --manifest-path tests/declaration/Cargo.toml --target x86_64-unknown-linux-gnu
export CARGO_TARGET_RISCV64GC_UNKNOWN_NONE_ELF_RUSTFLAGS='-Crelocation-model=static -Cforce-frame-pointers=yes -Ccode-model=medium'
cargo build --locked --target riscv64gc-unknown-none-elf -p router-server --bin router
cargo build --locked --release --target riscv64gc-unknown-none-elf -p router-server --bin router
```

These host suites prove the handoff helpers and declaration fields only. They do
not claim PLIC/IRQ, Kernel, System, hardware, or real-image acceptance. Source
intake of the Router runtime remains a separate later PR group.

See `PROVENANCE.md` for the frozen Source anchor and the preserved-file policy.
Root's actual checks and limitations are recorded in `VALIDATION.md`.
