# heed (LMDB bindings)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Storage](../README.md) › [Embedded databases](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `C (Rust bindings)` · `MIT` · 🟢 active

> Typed Rust wrapper over LMDB; fastest reads in redb's comparison, 4.6x RocksDB, but slower batch writes and 3x the disk.

| | |
|---|---|
| **Beats** | RocksDB (rust-rocksdb 0.22) and SQLite (rusqlite 0.37, bundled, default rollback journal) as key-value stores |
| **Claimed result** | Random reads 637 vs 2911 ms for RocksDB (4.6x) and 4283 ms for SQLite (6.7x); 32-thread reads 125 vs 1100 ms (8.8x) and 26536 ms; bulk load 9232 vs 13969 ms (1.5x). Slower: batch writes 942 vs 451 ms, removals 10435 vs 6900 ms; uncompacted size 2.61 GiB vs 893.18 MiB (ratios derived from page data). |
| **Workload** | 5 million random 24-byte keys with 150-byte values: bulk load, 1,000 durable single-insert transactions, 100 batches of 1,000, 1 million random reads (1 to 32 threads), removals. |
| **Metric** | time |
| **Hardware** | AMD Ryzen 9 9950X3D, Samsung 9100 PRO NVMe |
| **Toolchain** | rustc 1.89, heed 0.22, cargo bench release profile |
| **Measured** | 2025-08-08 |
| **Adoption** | 920 stars |
| **Last commit** | 2026-10-26 |
| **Platforms** | Linux, macOS, Windows, BSDs; needs mmap |
| **Install** | cargo add heed |
| **Links** | [repository](https://github.com/meilisearch/heed) · [documentation](https://docs.rs/heed) · [benchmark](https://github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df92766/README.md) |

## Verdict

LMDB through heed had the fastest reads in redb's comparison, 4.6x faster than RocksDB and 6.7x faster than SQLite single-threaded, and the fastest bulk load. It was 2.1x slower than RocksDB on batch writes and 1.5x slower on removals, and used 3x the disk.

**Why tier A:** Third-party benchmark (redb's author) with public source, pinned toolchain, and stated CPU and SSD.

## Use it when

- Read-dominated workloads, especially many concurrent readers
- Bulk loading a dataset that is then mostly read

## Skip it when

- Write-heavy or delete-heavy workloads, where RocksDB and fjall were faster
- Disk space is tight: LMDB used about 3x RocksDB's space
- You need the data set to exceed the configured map size without planning for it

## How it is fast

LMDB memory-maps the whole database and serves reads straight from the OS page cache with no user-space cache or copy. Its copy-on-write B+tree lets readers proceed lock-free alongside one writer, which is why read throughput scales with threads.

## What it costs

- Opening an environment is unsafe in heed because the memory map must not be changed by other code
- Fixed maximum map size; one writer at a time
- C library underneath; no checksums on pages read from disk

## Problems with the evidence

- Measured by redb's author in August 2025 with heed 0.22; heed is now 0.23
- LMDB has no user-space cache, so the 4 GB cache given to the other engines did not apply to it; it relies on the OS page cache

## Evidence

<details>
<summary>9 quotes from the sources, checked 2026-10-28</summary>

> | random reads | 1138ms | **637ms** | 2911ms | 2177ms | 4283ms |
>
> — [github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df927…](https://github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df92766/README.md)

> | random reads (32 threads) | 410ms | **125ms** | 1100ms | 576ms | 26536ms |
>
> — [github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df927…](https://github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df92766/README.md)

> | bulk load | 17063ms | **9232ms** | 13969ms | 18619ms | 15341ms |
>
> — [github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df927…](https://github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df92766/README.md)

> | batch writes | 1595ms | 942ms | 451ms | **353ms** | 2625ms |
>
> — [github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df927…](https://github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df92766/README.md)

> | removals | 23297ms | 10435ms | 6900ms | **6004ms** | 10323ms |
>
> — [github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df927…](https://github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df92766/README.md)

> | uncompacted size | 4.00 GiB | 2.61 GiB | **893.18 MiB** | 1000.95 MiB | 1.09 GiB |
>
> — [github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df927…](https://github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df92766/README.md)

> Results collected on a Ryzen 9950X3D with Samsung 9100 PRO NVMe.
>
> — [github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df927…](https://github.com/cberner/redb/blob/88c66fbb145948a4a5d1e4408992e0978df92766/README.md)

> heed = "0.22"
>
> — [github.com/cberner/redb/blob/d6773ef554d059e3f8ca357fe4931f2c5995c3…](https://github.com/cberner/redb/blob/d6773ef554d059e3f8ca357fe4931f2c5995c340/crates/redb-bench/Cargo.toml)

> LMDB and SQLite do not perform checksum checks when retrieving pages from disk
>
> — [fjall-rs.github.io/post/fjall-3/](https://fjall-rs.github.io/post/fjall-3/)

</details>
