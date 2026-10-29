# Storage

<sub>[fastrust](../../README.md) › [Catalog](../README.md)</sub>

## [Embedded databases](embedded-databases/README.md)

**The default:** SQLite through rusqlite, and RocksDB bindings.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**fjall (pure-Rust LSM-tree)**](embedded-databases/fjall-rs-fjall.md) | Pure-Rust LSM-tree engine; beats RocksDB on batch writes and reads in one comparison, builds in seconds, compacts worse. | A | 🟡 Holds up in part |
| [**heed (LMDB bindings)**](embedded-databases/meilisearch-heed.md) | Typed Rust wrapper over LMDB; fastest reads in redb's comparison, 4.6x RocksDB, but slower batch writes and 3x the disk. | A | 🟡 Holds up in part |
| [**redb (copy-on-write B+tree KV store)**](embedded-databases/cberner-redb.md) | Pure-Rust ACID key-value store; 2.6x faster reads and single commits than RocksDB, but slower batch writes and larger files. | A | 🟡 Holds up in part |
| [**rusqlite bulk inserts (cached prepared statements, multi-row batches, one transaction)**](embedded-databases/rusqlite-rusqlite-prepared-batched-inserts.md) | Batching SQLite inserts through rusqlite's cached prepared statements cut 100M inserts from about 3 minutes to 34 seconds. | B | 🟡 Holds up in part |
| [**sled (Bw-tree-style embedded KV store)**](embedded-databases/spacejam-sled.md) | Lock-free embedded KV store whose LSM-like write claim fails: batch writes 1.9x slower than RocksDB, slowest bulk load tested. | A | ❌ Doesn't hold up |

## [SQL drivers and ORMs](sql-drivers/README.md)

**The default:** sqlx and Diesel.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**rusqlite (synchronous SQLite driver)**](sql-drivers/rusqlite-rusqlite.md) | Synchronous SQLite bindings; queries ran 11x to 49x faster than sqlx's async SQLite driver in Diesel's nightly suite. | A | ✅ Holds up |
| [**zero-postgres (sync and async PostgreSQL client)**](sql-drivers/elbaro-zero-postgres.md) | New PostgreSQL client that tops Diesel's select benchmarks: 1.3x faster than tokio-postgres, about 2x sqlx on 10,000 rows. | A | ✅ Holds up |
| [**Diesel (sync ORM and query builder)**](sql-drivers/diesel-rs-diesel.md) | Rust ORM with compile-time checked queries; loads 10,000-row results 1.2x to 1.4x faster than sqlx, ties it on single rows. | A | 🟡 Holds up in part |
| [**tokio-postgres (async PostgreSQL driver)**](sql-drivers/rust-postgres-rust-postgres.md) | Async PostgreSQL driver; 1.5x to 1.9x faster than sqlx on 10,000-row results, slower on one-row queries in the same suite. | A | 🟡 Holds up in part |

## [In-memory caches](caches/README.md)

**The default:** A HashMap with hand-written eviction, or the lru crate.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**quick_cache (S3-FIFO concurrent cache)**](caches/arthurprs-quick-cache.md) | Sharded S3-FIFO cache: up to 5.5x the hit ratio of Mutex<LRU> at similar per-op cost, about 4x faster than Mini Moka. | C | ✅ Holds up |
| [**moka (TinyLFU concurrent cache, with Mini Moka)**](caches/moka-rs-moka.md) | Caffeine-style cache with TTL and listeners; 4.5x the hit ratio of Mutex<LRU> on a search trace, but 7.3x the per-op cost. | B | 🟡 Holds up in part |
| [**stretto (Rust port of Ristretto)**](caches/al8n-stretto.md) | TinyLFU cache ported from Go's Ristretto; wins hit ratio only on small caches over skewed keys, and 0.8 measured broken. | C | 🟡 Holds up in part |
