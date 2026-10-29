# SQL drivers and ORMs

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Storage](../README.md)</sub>

**The default:** sqlx and Diesel.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**rusqlite (synchronous SQLite driver)**](rusqlite-rusqlite.md) | Synchronous SQLite bindings; queries ran 11x to 49x faster than sqlx's async SQLite driver in Diesel's nightly suite. | A | ✅ Holds up |
| [**zero-postgres (sync and async PostgreSQL client)**](elbaro-zero-postgres.md) | New PostgreSQL client that tops Diesel's select benchmarks: 1.3x faster than tokio-postgres, about 2x sqlx on 10,000 rows. | A | ✅ Holds up |
| [**Diesel (sync ORM and query builder)**](diesel-rs-diesel.md) | Rust ORM with compile-time checked queries; loads 10,000-row results 1.2x to 1.4x faster than sqlx, ties it on single rows. | A | 🟡 Holds up in part |
| [**tokio-postgres (async PostgreSQL driver)**](rust-postgres-rust-postgres.md) | Async PostgreSQL driver; 1.5x to 1.9x faster than sqlx on 10,000-row results, slower on one-row queries in the same suite. | A | 🟡 Holds up in part |
