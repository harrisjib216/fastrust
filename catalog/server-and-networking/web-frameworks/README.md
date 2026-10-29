# Web frameworks

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Server and networking](../README.md)</sub>

**The default:** axum on hyper and Tokio.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**actix-web 4**](actix-actix-web.md) | Mature Rust web framework that beats default-config axum 1.1-2.6x on small-request benchmarks; the lead shrinks to ~15% once a database is involved. | A | 🟡 Holds up in part |
| [**hyper (raw server, without axum)**](hyperium-hyper.md) | Serving straight from hyper beats axum 1.4-3.5x on trivial handlers, but the gap is 6% on JSON and reverses on TechEmpower fortunes. | A | 🟡 Holds up in part |
| [**may_minihttp (HTTP/1.1 server on stackful coroutines)**](xudong-huang-may-minihttp.md) | Minimal HTTP/1.1 server on the may coroutine runtime; tops TechEmpower plaintext but leads axum by only 14-19% on JSON and fortunes. | A | 🟡 Holds up in part |
| [**ntex (web framework, actix-web fork)**](ntex-rs-ntex.md) | actix-style web framework that beats default-config axum 1.3-2.2x on HttpArena but is within 2-9% of thread-per-core axum on TechEmpower. | A | 🟡 Holds up in part |
| [**xitca-web (HTTP library and web framework)**](hfqr-xitca-web.md) | Rust web framework with its own HTTP stack; ahead of axum on pipelined plaintext in TechEmpower, level on JSON and fortunes. | A | 🟡 Holds up in part |
