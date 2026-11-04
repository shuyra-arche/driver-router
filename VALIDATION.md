# Router provider validation

Root independently validated the candidate on 2026-11-04 using
nightly-2026-09-04, with four build jobs, the shared native-build lock and a
6 GiB minimum free-space guard. Source remained at
`c975cec91273261509c8f9f715643830c2fed5c5` and Driver at
`e75b1e232f828666956fb7e02c61a7d84f13eb66`, both clean.

- All 35 original runtime items, the startup body, declaration initializer and
  four original handoff case bodies match parsed Source ASTs after the documented
  facade/module substitutions. `Cell` alone gains narrowly internal
  `pub(super)` visibility. Runtime modules remain private behind `serve()`.
- The five API and four client files are byte-identical to their canonical
  revisions. Locked online/offline metadata selects Git router-api `94fec223…`
  and Git router-client `aa3033a…` through Git driver-shared `e75b1e2…` in the
  new runtime closure. Frozen local workspace members are outside that closure.
  No Source programs or driver-support dependency is introduced. Existing lock
  entries retain their resolved identities; disambiguated labels are normalized
  only by the verifier.
- Host debug/release library builds passed. The four preserved handoff helper
  cases and two declaration/BUILD contract cases passed, six distinct cases.
  Fixture types remain separated from production helpers and from each other.
- Actual RISC-V debug and release `router` executables were built with the
  Source flags documented in README. Cargo artifact records identify the new
  `router-server` manifest and executable. Both are ELF64 little-endian RISC-V
  ET_EXEC, load from `0x10000`, have executable entry points and page-aligned
  LOAD segments, no interpreter or unresolved symbols, and exactly one each of
  `_start`, `clean_ret`, panic, alloc, dealloc, realloc and alloc-zeroed symbols.
  No host std symbols were found. Compressed ELF copies round-trip exactly.

Initial failures are retained in Root evidence: the copied handoff tests had a
redundant nested module and failed compilation with exit 101; the same worker
fixed module glue without changing any assertion, and the exact tests and AST
check then passed. Two Root verification invocations failed because of an
output-folder routing error and Cargo lock-label parsing; those verifier issues
were corrected without changing product code or dependency pins, and both
checks passed. The history is not presented as an uninterrupted green run.

A fresh genuine opencode-go/deepseek-v4.1-flash namer, worker and reviewer group
owns this provider slice. Root owns commands, locks, independent acceptance and
Git. Models use file tools; no model shell command is forwarded. Review receipts
are retained with Root evidence. Public full-SHA fresh-checkout receipts will be
retained after publication and before any consumer.

This is provider validation. No Source/Driver aggregate intake, QEMU execution,
PLIC/IRQ behavior, hardware acceptance, complete historical scenario matrix or
new Source image acceptance is claimed. Before a consumer pins the published
revision, Root must repeat locked builds, tests, closure and real ELF checks from
a clean public checkout with a separate target directory. Consumers and their
PRs use new role groups and the serial Source/lock/QEMU queue. Do not merge this
candidate automatically.
