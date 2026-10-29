# redb (copy-on-write B+tree KV store)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Storage](../README.md) › [Embedded databases](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Pure-Rust ACID key-value store; 2.6x faster reads and single commits than RocksDB, but slower batch writes and larger files.

| | |
|---|---|
| **Beats** | RocksDB (rust-rocksdb 0.22) and SQLite (rusqlite 0.37, bundled, default rollback journal) as key-value stores |
| **Claimed result** | vs RocksDB: random reads 1138 vs 2911 ms (2.6x faster), individual writes 920 vs 2432 ms (2.6x faster); vs SQLite: random reads 1138 vs 4283 ms (3.8x), individual writes 920 vs 7040 ms (7.7x). Slower than RocksDB on batch writes (1595 vs 451 ms), removals (23297 vs 6900 ms) and bulk load (17063 vs 13969 ms); uncompacted size 4.00 GiB vs 893.18 MiB (ratios derived from page data). |
| **Workload** | 5 million random 24-byte keys with 150-byte values: bulk load, 1,000 durable single-insert transactions, 100 batches of 1,000, 1 million random reads and range reads (1 to 32 threads), removals; 4 GB cache. |
| **Metric** | time |
| **Hardware** | AMD Ryzen 9 9950X3D, Samsung 9100 PRO NVMe |
| **Toolchain** | rustc 1.89 (rust-toolchain file at the benchmark commit), cargo bench release profile |
| **Measured** | 2025-08-08 |
| **Adoption** | 4.9k stars |
| **Last commit** | 2026-10-26 |
| **Platforms** | Any Rust target with a file system; no_std with a custom storage backend |
| **Install** | cargo add redb |
| **Links** | [repository](https://github.com/cberner/redb) · [documentation](https://docs.rs/redb) · [benchmark](https://github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df92766/README.md) |

## Verdict

redb is 2.6x faster than RocksDB on random reads and durable single-row commits, and beats SQLite on reads and small writes. It loses to RocksDB on batch writes (3.5x slower), removals (3.4x) and bulk load (1.2x), loses to SQLite on bulk load and removals, its files are 4.6x larger than RocksDB's, and LMDB beats it on every read test.

**Why tier A:** Benchmark source is in the repository, the toolchain is pinned by rust-toolchain (1.89), and CPU and SSD are stated.

## Use it when

- Read-heavy embedded storage with many small durable transactions
- You want a pure-Rust store with a typed BTreeMap-like API instead of RocksDB's C++ build

## Skip it when

- Write-heavy ingestion in large batches, or delete-heavy workloads: RocksDB and fjall are faster
- Disk space matters: files were 4.6x larger than RocksDB's before compaction
- Pure read latency is all that matters: LMDB (heed) was faster on every read test

## How it is fast

Data lives in copy-on-write B+trees with MVCC, so readers never block and a point read is a short tree walk with no compaction or bloom-filter work. A durable commit writes the changed pages and flips a root pointer. The cost is write amplification on bulk and batch writes and no compression on disk.

## What it costs

- No on-disk compression; space use is the highest of the engines tested
- Single writer at a time per database (multi-writer mode is newly stabilized)

## Problems with the evidence

- First-party benchmark written by redb's author
- README numbers were collected in August 2025 with redb 2.6.0; redb is now 4.4.0 and the benchmark code has changed since
- SQLite ran with defaults (rollback journal, no WAL, synchronous FULL on durable writes), which understates it

## Evidence

<details>
<summary>10 quotes from the sources, checked 2026-10-28</summary>

> redb has similar performance to other top embedded key-value stores such as lmdb and rocksdb
>
> — [github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df927…](https://github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df92766/README.md)

> | individual writes | **920ms** | 1598ms | 2432ms | 3488ms | 7040ms |
>
> — [github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df927…](https://github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df92766/README.md)

> | random reads | 1138ms | **637ms** | 2911ms | 2177ms | 4283ms |
>
> — [github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df927…](https://github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df92766/README.md)

> | batch writes | 1595ms | 942ms | 451ms | **353ms** | 2625ms |
>
> — [github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df927…](https://github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df92766/README.md)

> | removals | 23297ms | 10435ms | 6900ms | **6004ms** | 10323ms |
>
> — [github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df927…](https://github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df92766/README.md)

> | bulk load | 17063ms | **9232ms** | 13969ms | 18619ms | 15341ms |
>
> — [github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df927…](https://github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df92766/README.md)

> | uncompacted size | 4.00 GiB | 2.61 GiB | **893.18 MiB** | 1000.95 MiB | 1.09 GiB |
>
> — [github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df927…](https://github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df92766/README.md)

> Results collected on a Ryzen 9950X3D with Samsung 9100 PRO NVMe.
>
> — [github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df927…](https://github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df92766/README.md)

> 1.89
>
> — [github.com/cberner/redb/blob/d6773ef554d059e3f8ca357fe4931f2c5995c3…](https://github.com/cberner/redb/blob/d6773ef554d059e3f8ca357fe4931f2c5995c340/rust-toolchain)

> rocksdb = { version = "0.22.0", default-features = false, features = ["lz4"] }
>
> — [github.com/cberner/redb/blob/d6773ef554d059e3f8ca357fe4931f2c5995c3…](https://github.com/cberner/redb/blob/d6773ef554d059e3f8ca357fe4931f2c5995c340/crates/redb-bench/Cargo.toml)

</details>
