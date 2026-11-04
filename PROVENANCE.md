# Provenance

Original Source: https://github.com/anreonyr/sqware.git
Frozen Source for this provider slice: `c975cec91273261509c8f9f715643830c2fed5c5`.
Candidate branch base: `aa3033a7bd25b5d4199a50a6c00538475b62ef4c`.

This Router Unit revision adds the `router-server` library/binary, the Router
declaration (`UNIT`, `BUILD`), and two standalone host suites to the existing
frozen API and client. Runtime code is taken from Source `c975cec…` at
`programs/src/driver/router/`; facade imports were normalized to public providers
and the two multi-structure files were split. The original main body now lives
in one public `serve()` library entry behind private runtime modules, with a thin
`mold`/`execution` binary entry. The READY reference is explicitly `::unit::READY_MARK`
because `env::unit` is already imported in the same file.

## Preserved files

The five API files (`api/Cargo.toml`, `api/src/lib.rs`, `api/src/declaration.rs`,
`api/src/frame.rs`, `api/src/marks.rs`) remain byte-identical to the previously
published canonical `router-api` revision
`94fec223f3c5df6c99a49527a65d016d88de3af4`.

The four client files (`client/Cargo.toml`, `client/src/lib.rs`,
`client/src/line.rs`, `client/src/handoff.rs`) remain byte-identical to the public
Router Unit revision `aa3033a7bd25b5d4199a50a6c00538475b62ef4c`.
Its original Source content anchor is
`ac5b61d8880c499f8cee4c60929c06cc9f4820d8`; the files keep the canonical
`router-api` Git pin `94fec…` and no local protocol path.

## Added provider files

- `server/` — Source `programs/src/driver/router/{main,mod,core,adapt,dev}`.
  `core/lines/cell.rs` and `adapt/event/desk/reply.rs` are the two required
  one-structure extractions; all runtime bodies, literals, field order, waits,
  drop paths, and messages are otherwise preserved.
- `declaration/` — the Source Router `PROGRAM` initializer renamed to `UNIT` with
  `crate::unit::` → `unit::`, plus the real `BUILD` for image `router`.
- `tests/handoff/` — the four original handoff cases, with the real client/server
  helper files projected by corrected relative `#[path]`s.
- `tests/declaration/` — field-contract checks for `UNIT` and `BUILD`.

Root owns generated locks, public clean-checkout proof, actual RV ELF/symbol
checks, provenance hashes, commit, and publication. The worker ran no shell,
Git, network, or build command.

The Source root has no project LICENSE. This snapshot invents none and does not
rewrite Source or prior public histories.
