# Router Unit

This snapshot contains the pure Router API prerequisite and the Router client.

The five API files (`api/Cargo.toml`, `api/src/lib.rs`, `api/src/declaration.rs`,
`api/src/frame.rs`, `api/src/marks.rs`) are the exact original Source `293c15fc…`
Router API files, published at canonical API revision
`94fec223f3c5df6c99a49527a65d016d88de3af4`, and must remain byte-identical.

The four client files (`client/Cargo.toml`, `client/src/lib.rs`,
`client/src/line.rs`, `client/src/handoff.rs`) are byte-identical to Source
`ac5b61d8880c499f8cee4c60929c06cc9f4820d8`. The client keeps the original
canonical `router-api` Git pin `94fec…`; it has no `path` dependency on the local
API. `client` is a workspace member beside `api`.

This repository does **not** claim that the Router runtime has moved. Router
server, declaration, shared Context/Device/Fail support and runtime ownership
remain Source and are pending Unit intake. No fake server, declaration, UnitFile,
or runtime image is present here.

The public repository is `shuyra-arche/router`, verified available and pushable.
Consumers use the complete published revision after public clean-checkout proof;
no placeholder revision or local path back-link to Driver/Source is used.
The Router API is a Hub prerequisite. The client is a prerequisite for future
explicit shared Context/Device/Fail support and Router server extraction; Hub
does not depend on it. Root owns provenance, hashes, generated locks,
native/public proof, and publication.
