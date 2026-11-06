# Router Unit

Router owns the external-interrupt routing service for Shuyra. Its server claims
PLIC, device-tree and interrupt resources through Hub.

## Layout

- `api/`: interrupt-routing protocol and marks.
- `client/`: line and handoff client.
- `server/`: private device/core/adapters, public `serve()`, and the `router` binary.
- `unit.toml`: startup policy, requiring and following Hub.
- `declaration/`: compatibility Rust Unit and build declarations.
- `tests/handoff/`, `tests/declaration/`, `tests/unit-document/`: host workspaces.

System completes its internal bootstrap before ordinary Units start.
The API and client members retain their published Git dependency identities;
do not replace individual pins without checking the consumer's complete type graph.

## Build and test

Install Rust through rustup; `rust-toolchain.toml` pins nightly-2026-09-04 and RV64.

```sh
for suite in handoff declaration unit-document; do
  cargo test --locked --manifest-path "tests/$suite/Cargo.toml" --target x86_64-unknown-linux-gnu || exit
done
CARGO_TARGET_RISCV64GC_UNKNOWN_NONE_ELF_RUSTFLAGS='-Crelocation-model=static -Cforce-frame-pointers=yes -Ccode-model=medium' \
  cargo build --locked --release --target riscv64gc-unknown-none-elf -p router-server --bin router
```

Uncached dependencies require network access. Host tests cover helpers, declarations
and encoding. PLIC/IRQ delivery requires composed runtime checks through
[Vanllia](https://github.com/shuyra-arche/vanllia).

See [source attribution](PROVENANCE.md) and the
[organization contribution guide](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md).
