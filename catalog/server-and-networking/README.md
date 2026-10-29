# Server and networking

<sub>[fastrust](../../README.md) › [Catalog](../README.md)</sub>

## [Web frameworks](web-frameworks/README.md)

**The default:** axum on hyper and Tokio.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**actix-web 4**](web-frameworks/actix-actix-web.md) | Mature Rust web framework that beats default-config axum 1.1-2.6x on small-request benchmarks; the lead shrinks to ~15% once a database is involved. | A | 🟡 Holds up in part |
| [**hyper (raw server, without axum)**](web-frameworks/hyperium-hyper.md) | Serving straight from hyper beats axum 1.4-3.5x on trivial handlers, but the gap is 6% on JSON and reverses on TechEmpower fortunes. | A | 🟡 Holds up in part |
| [**may_minihttp (HTTP/1.1 server on stackful coroutines)**](web-frameworks/xudong-huang-may-minihttp.md) | Minimal HTTP/1.1 server on the may coroutine runtime; tops TechEmpower plaintext but leads axum by only 14-19% on JSON and fortunes. | A | 🟡 Holds up in part |
| [**ntex (web framework, actix-web fork)**](web-frameworks/ntex-rs-ntex.md) | actix-style web framework that beats default-config axum 1.3-2.2x on HttpArena but is within 2-9% of thread-per-core axum on TechEmpower. | A | 🟡 Holds up in part |
| [**xitca-web (HTTP library and web framework)**](web-frameworks/hfqr-xitca-web.md) | Rust web framework with its own HTTP stack; ahead of axum on pipelined plaintext in TechEmpower, level on JSON and fortunes. | A | 🟡 Holds up in part |

## [HTTP clients](http-clients/README.md)

**The default:** reqwest.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**ureq (blocking HTTP client)**](http-clients/algesten-ureq.md) | Blocking HTTP client with a small dependency tree; no measured speed advantage over reqwest for sequential requests. | B | ❌ Doesn't hold up |

## [Protocol Buffers and RPC](protobuf-and-rpc/README.md)

**The default:** prost and tonic.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**buffa (protobuf with zero-copy view types)**](protobuf-and-rpc/anthropics-buffa.md) | Pure-Rust protobuf with editions support; about even with prost on owned types, 1.1-2.8x faster decode through zero-copy views. | A | 🟡 Holds up in part |
| [**pajamax (synchronous gRPC server)**](protobuf-and-rpc/wubingzheng-pajamax.md) | Thread-per-connection gRPC server with a gRPC-specific HTTP/2 parser; 3-30% faster than tonic in a third-party run, not the 10x its author cites. | A | 🟡 Holds up in part |
| [**wtx (gRPC server)**](protobuf-and-rpc/c410-f3r-wtx.md) | Collection of web transports with its own HTTP/2 and gRPC stack; beat tonic in its author's grpc_bench run, lost by ~1.8x in the maintainer's. | A | 🟡 Holds up in part |
| [**quick-protobuf (borrowing protobuf decoder)**](protobuf-and-rpc/tafia-quick-protobuf.md) | Protobuf crate whose generated types borrow strings and bytes from the input; 2018 numbers show faster reads than prost, slower writes. | C | 🟡 Holds up in part |
