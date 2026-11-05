# Router provider validation

## Public Unit document API adoption — 2026-11-05

The document suite now pins the published Unit API at
`ad50b803c3bd5cd2ef90010d80badf23551e88d5` and owns
`tests/unit-document/Cargo.lock`. Its `legacy-unit` alias remains pinned to
`90396f0fda376e0693fb1b9553aa21864d241057` so native enum comparisons use the
same types as `router-declaration`. Production manifests and all pre-existing
locks retain their original pins.

The worker sourced `/workspace/.vanllia-tools/env.sh`, generated the new lock
offline, and ran the real `tests/unit-document/Cargo.toml` with `--locked
--offline`, target `x86_64-unknown-linux-gnu` and the fresh independent
`/tmp/router-unit-public-target` directory. All four tests passed without a
Cargo patch or staged manifest. Locked offline Cargo metadata resolves both
Unit versions to their full Git revisions and Cargo Git checkout paths; neither
resolves to `/workspace/unit` or another local replacement. `rustfmt --check`
and `git diff --check` passed. No policy field or test assertion changed; only
the legacy type paths changed to the explicit alias.

The independent reviewer repeated all four tests with `--locked --offline`,
the real manifest, no patch and a fresh ignored `target/reviewer-public`
directory; all four passed. Separate locked offline metadata confirmed the two
full public Unit revisions resolve to Cargo Git checkouts. Review verified that
normalizing the `legacy_unit` type paths leaves the test logic and assertions
unchanged, and that only this follow-up's five files changed. Reviewer
`rustfmt --check` and `git diff --check` also passed.

The reviewer's first fresh build under `/tmp/router-reviewer-public-target`
failed while compiling dependencies because the separate `/tmp` filesystem was
full. After removing only reviewer-owned generated build artifacts, the build
was repeated on the `/workspace` filesystem with the ignored target directory
above and passed. No test or dependency setting changed to resolve that failure.

This resolves the unpublished-API and missing-new-lock limitation of the initial
local checkpoint below. No new RISC-V build, old-scenario migration, QEMU,
PLIC/IRQ or hardware run was performed in this dependency-only follow-up.

## Initial local Unit document migration — 2026-11-05

The repository now owns `unit.toml` as the current schema-1 runtime policy.
The legacy declaration initializer and BUILD contract are unchanged; only its
module documentation identifies the compatibility role. API, client, server,
existing scenarios, production dependency pins and all existing locks are
unchanged.

Using `source /workspace/.vanllia-tools/env.sh` (nightly-2026-10-09), the worker
ran these checks offline with the independent
`/tmp/router-unit-runtime-target` directory and
`/tmp/router-unit-runtime-target.lock`:

- A real Rust probe read the old declaration and canonical provider constants:
  SERVICE name `service`, namespace mark `14715478334910145156`, Ops `279`,
  ENTRY name `entry`, ENTRY mark `13702911872889301267`, STORE bits `2`, and
  both Unit/Router READY marks `9887014109901452468`. Identity grants are empty.
- The four `tests/unit-document` tests passed with the command-line Unit patch
  to `/workspace/unit`, the manifest staged under ignored `target/`, and its
  staged lock. The final run used `--locked`. They compare every policy field
  with native Rust, check both quoted decimal and hex full-width marks, compare
  the entire document after binary record roundtrip, verify borrowed strings,
  and check the existing Cargo bin/BUILD contract.
- The two legacy declaration tests and four preserved handoff tests passed
  with their existing locks and public dependency pins, without a Unit patch.
  The declaration manifest retains its existing unused-dependency warning.
- The independent reviewer probe passed a separate native field comparison,
  a complete 99-byte record roundtrip, borrowed-id check and
  `runtime::UnitFile` conversion. It checked the intentional mapping from
  Operator bootstrap plus Hub ordering to `requires = ["hub"]` and
  `after = ["hub"]`.
- `rustfmt --check` for the new test file and `git diff --check` passed.

The initial temporary native probe did not compile because `Access::bits()`
returns `Permission`; reading its `.bits()` produced the intended `u32` value.
The first new-suite compile failed because a generic Scope equality assertion
required unrelated Debug/PartialEq bounds; it was replaced with a direct
Subtree variant assertion. Both corrected commands passed, with no policy or
security assertion removed.

At checkpoint `6a3dca3`, the shared document/record API had not been published;
the formal Unit revision and the new suite's lock were still pending. The local
patch was not committed, and that checkpoint's checks did not establish an
unpatched public-dependency run for the new suite. The public adoption above
supersedes this limitation. No new RISC-V executable build, Source/Driver intake,
QEMU, PLIC/IRQ or hardware run was performed for this declaration-only change;
the earlier provider build evidence remains historical below.

## Earlier provider validation

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
