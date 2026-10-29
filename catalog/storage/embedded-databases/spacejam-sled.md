# sled (Bw-tree-style embedded KV store)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Storage](../README.md) › [Embedded databases](README.md)</sub>

**Tier A** (reproducible public benchmark) · ❌ Doesn't hold up · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Lock-free embedded KV store whose LSM-like write claim fails: batch writes 1.9x slower than RocksDB, slowest bulk load tested.

| | |
|---|---|
| **Beats** | RocksDB through rust-rocksdb 0.22 (the LSM-tree its README compares itself to), and SQLite through rusqlite |
| **Claimed result** | Claimed: LSM-tree-like writes and B+tree-like reads. Measured (sled 0.34.7): batch writes 853 ms vs 451 ms RocksDB and 353 ms fjall; bulk load 24971 vs 13969 ms; random reads 1601 vs 2911 ms RocksDB but 1138 ms redb and 637 ms LMDB; uncompacted size 2.13 GiB vs 893.18 MiB. |
| **Workload** | 5 million random 24-byte keys with 150-byte values: bulk load, 100 batches of 1,000, 1 million random reads, removals. |
| **Metric** | time |
| **Hardware** | AMD Ryzen 9 9950X3D, Samsung 9100 PRO NVMe |
| **Toolchain** | rustc 1.89, sled 0.34.7, cargo bench release profile |
| **Measured** | 2025-08-08 |
| **Adoption** | 9.1k stars |
| **Last commit** | 2026-04-04 |
| **Platforms** | Targets with std file I/O; Linux, macOS and Windows supported |
| **Install** | cargo add sled |
| **Links** | [repository](https://github.com/spacejam/sled) · [documentation](https://docs.rs/sled) · [benchmark](https://github.com/cberner/redb/blob/d6773ef554d059e3f8ca357fe4931f2c5995c340/README.md) |

## Verdict

sled claims LSM-like write speed with B+tree-like reads. Against an actual LSM, its batch writes were 1.9x slower than RocksDB and 2.4x slower than fjall, and its bulk load was the slowest tested. Its reads beat RocksDB but trail redb and LMDB. fjall's author excluded it from durable-write tests because it does not reliably fsync, and redb dropped it over open security advisories and no recent release.

**Why tier A:** Third-party benchmark (redb's author) with public source, pinned toolchain, and stated CPU and SSD.

## Use it when

- Existing code already on sled 0.34 that is read-heavy and not durability-critical

## Skip it when

- New projects: the stable 0.34.7 line dates from 2021 and 1.0 is still in alpha
- Write-heavy workloads: RocksDB and fjall were faster
- You need every acknowledged write to be on disk

## How it is fast

A lock-free tree over a log-structured page cache that writes page deltas instead of whole pages. In practice this gives decent reads but heavy space use and slower writes than real LSM engines.

## What it costs

- The README itself says sled is beta, sometimes uses too much space, and points reliability-focused users to SQLite
- On-disk format will change before 1.0 and need manual migration

## Problems with the evidence

- Only one third-party run with sled in it (August 2025); redb has since removed sled from its comparison
- sled's own claim of a billion operations per minute has no published harness or hardware

## Evidence

<details>
<summary>11 quotes from the sources, checked 2026-10-28</summary>

> with [traditional B+ tree](https://en.wikipedia.org/wiki/B%2B_tree)-like read performance
>
> — [github.com/spacejam/sled/blob/eb7ca716c8ded622c13943416064bcd4922aa…](https://github.com/spacejam/sled/blob/eb7ca716c8ded622c13943416064bcd4922aa40b/README.md)

> if reliability is your primary constraint, use SQLite. sled is beta.
>
> — [github.com/spacejam/sled/blob/eb7ca716c8ded622c13943416064bcd4922aa…](https://github.com/spacejam/sled/blob/eb7ca716c8ded622c13943416064bcd4922aa40b/README.md)

> if storage price performance is your primary constraint, use RocksDB. sled uses too much space sometimes.
>
> — [github.com/spacejam/sled/blob/eb7ca716c8ded622c13943416064bcd4922aa…](https://github.com/spacejam/sled/blob/eb7ca716c8ded622c13943416064bcd4922aa40b/README.md)

> | | redb | lmdb | rocksdb | sled | fjall | sqlite |
>
> — [github.com/cberner/redb/blob/d6773ef554d059e3f8ca357fe4931f2c5995c3…](https://github.com/cberner/redb/blob/d6773ef554d059e3f8ca357fe4931f2c5995c340/README.md)

> | batch writes | 1595ms | 942ms | 451ms | 853ms | **353ms** | 2625ms |
>
> — [github.com/cberner/redb/blob/d6773ef554d059e3f8ca357fe4931f2c5995c3…](https://github.com/cberner/redb/blob/d6773ef554d059e3f8ca357fe4931f2c5995c340/README.md)

> | bulk load | 17063ms | **9232ms** | 13969ms | 24971ms | 18619ms | 15341ms |
>
> — [github.com/cberner/redb/blob/d6773ef554d059e3f8ca357fe4931f2c5995c3…](https://github.com/cberner/redb/blob/d6773ef554d059e3f8ca357fe4931f2c5995c340/README.md)

> | random reads | 1138ms | **637ms** | 2911ms | 1601ms | 2177ms | 4283ms |
>
> — [github.com/cberner/redb/blob/d6773ef554d059e3f8ca357fe4931f2c5995c3…](https://github.com/cberner/redb/blob/d6773ef554d059e3f8ca357fe4931f2c5995c340/README.md)

> | uncompacted size | 4.00 GiB | 2.61 GiB | **893.18 MiB** | 2.13 GiB | 1000.95 MiB | 1.09 GiB |
>
> — [github.com/cberner/redb/blob/d6773ef554d059e3f8ca357fe4931f2c5995c3…](https://github.com/cberner/redb/blob/d6773ef554d059e3f8ca357fe4931f2c5995c340/README.md)

> sled = "0.34.7"
>
> — [github.com/cberner/redb/blob/d6773ef554d059e3f8ca357fe4931f2c5995c3…](https://github.com/cberner/redb/blob/d6773ef554d059e3f8ca357fe4931f2c5995c340/crates/redb-bench/Cargo.toml)

> Sled is not included because it does not reliably fsync
>
> — [fjall-rs.github.io/post/fjall-3/](https://fjall-rs.github.io/post/fjall-3/)

> It has several security advisories open
>
> — [github.com/cberner/redb/commit/237dff1e1ed88b4bdee7b6c77cea676efdff…](https://github.com/cberner/redb/commit/237dff1e1ed88b4bdee7b6c77cea676efdffa770)

</details>
