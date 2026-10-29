# Embedded databases

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Storage](../README.md)</sub>

**The default:** SQLite through rusqlite, and RocksDB bindings.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**fjall (pure-Rust LSM-tree)**](fjall-rs-fjall.md) | Pure-Rust LSM-tree engine; beats RocksDB on batch writes and reads in one comparison, builds in seconds, compacts worse. | A | 🟡 Holds up in part |
| [**heed (LMDB bindings)**](meilisearch-heed.md) | Typed Rust wrapper over LMDB; fastest reads in redb's comparison, 4.6x RocksDB, but slower batch writes and 3x the disk. | A | 🟡 Holds up in part |
| [**redb (copy-on-write B+tree KV store)**](cberner-redb.md) | Pure-Rust ACID key-value store; 2.6x faster reads and single commits than RocksDB, but slower batch writes and larger files. | A | 🟡 Holds up in part |
| [**rusqlite bulk inserts (cached prepared statements, multi-row batches, one transaction)**](rusqlite-rusqlite-prepared-batched-inserts.md) | Batching SQLite inserts through rusqlite's cached prepared statements cut 100M inserts from about 3 minutes to 34 seconds. | B | 🟡 Holds up in part |
| [**sled (Bw-tree-style embedded KV store)**](spacejam-sled.md) | Lock-free embedded KV store whose LSM-like write claim fails: batch writes 1.9x slower than RocksDB, slowest bulk load tested. | A | ❌ Doesn't hold up |
