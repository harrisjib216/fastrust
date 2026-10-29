# rusqlite bulk inserts (cached prepared statements, multi-row batches, one transaction)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Storage](../README.md) › [Embedded databases](README.md)</sub>

**Tier B** (independent measurement) · 🟡 Holds up in part · Open source · `Rust + C` · `MIT` · 🟢 active

> Batching SQLite inserts through rusqlite's cached prepared statements cut 100M inserts from about 3 minutes to 34 seconds.

| | |
|---|---|
| **Beats** | rusqlite inserting one row per execute call in a loop, with the same SQLite PRAGMAs |
| **Claimed result** | 100M rows: naive per-row loop about 3 minutes; reused prepared statement about a minute; prepared statement with 50-row batches 34.3 seconds (about 5x vs the loop, derived from page data). |
| **Workload** | Insert 100 million rows of random (area code, age, active flag) into a 4-column SQLite table, single writer, inside one transaction for the batched versions. |
| **Metric** | time |
| **Hardware** | MacBook Pro 2019, 2.4 GHz quad-core Intel i5, 8 GB RAM, 256 GB SSD |
| **OS** | macOS Big Sur 11.1 |
| **Toolchain** | rusqlite 0.25.3; rustc version not stated |
| **Measured** | 2021-07-17 |
| **Adoption** | 4.5k stars |
| **Last commit** | 2026-10-27 |
| **Platforms** | Any target SQLite builds on |
| **Install** | cargo add rusqlite (use Connection::transaction and prepare_cached) |
| **Links** | [repository](https://github.com/rusqlite/rusqlite) · [documentation](https://docs.rs/rusqlite/latest/rusqlite/struct.Connection.html#method.prepare_cached) · [benchmark](https://avi.im/blag/2021/fast-sqlite-inserts/) |

## Verdict

Reusing a prepared statement and inserting 50 rows per statement cut 100 million inserts from about 3 minutes to 34.3 seconds. But every run also turned off SQLite's journal and fsync, which the author says not to use in production, and the data is from 2021 on a laptop.

**Why tier B:** Independent of the rusqlite authors, with public code and the machine stated, but no rustc version or flags.

## Use it when

- Loading or generating large SQLite databases (test fixtures, imports, ETL)
- Any hot insert path: prepared-statement reuse and transactions are safe and cheap wins

## Skip it when

- Production databases that must survive a crash: do not copy journal_mode OFF or synchronous 0
- Few, scattered writes where per-statement overhead is not the bottleneck

## How it is fast

Preparing the INSERT once with prepare_cached skips SQL parsing and planning on every row, and a 50-row VALUES list amortizes each statement step across many rows. Wrapping everything in one transaction avoids a commit per row. The PRAGMAs (journal_mode OFF, synchronous 0, large cache, exclusive locking) remove most of the remaining I/O cost.

## What it costs

- Batched multi-row statements need careful parameter handling and a fixed batch size
- One long transaction holds the write lock for its whole duration

## Problems with the evidence

- 2021 data on a 2019 laptop with rusqlite 0.25.3
- All Rust variants ran with journaling and fsync disabled, so the absolute times overstate what a durable setup gets
- An in-memory database ran only about 2 seconds faster, so disk I/O was not the bottleneck once durability was off

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> However, I included all the SQLite optimisations. This version took about 3 minutes.
>
> — [avi.im/blag/2021/fast-sqlite-inserts/](https://avi.im/blag/2021/fast-sqlite-inserts/)

> switched to prepared statements and inserted the rows in a loop, but reusing the prepared statement. This version took about only a minute.
>
> — [avi.im/blag/2021/fast-sqlite-inserts/](https://avi.im/blag/2021/fast-sqlite-inserts/)

> I used prepared statements and inserted them in a batch of 50 rows. To insert 100M rows, took 34.3 seconds.
>
> — [avi.im/blag/2021/fast-sqlite-inserts/](https://avi.im/blag/2021/fast-sqlite-inserts/)

> The machine I am using is MacBook Pro, 2019 (2.4 GHz Quad Core i5, 8GB, 256GB SSD, Big Sur 11.1)
>
> — [avi.im/blag/2021/fast-sqlite-inserts/](https://avi.im/blag/2021/fast-sqlite-inserts/)

> will result in no rollback journal, thus we cannot go back if any of the transactions fail.
>
> — [avi.im/blag/2021/fast-sqlite-inserts/](https://avi.im/blag/2021/fast-sqlite-inserts/)

> SQLite does not care about writing to disk reliably and hands off that responsibility to the OS.
>
> — [avi.im/blag/2021/fast-sqlite-inserts/](https://avi.im/blag/2021/fast-sqlite-inserts/)

> Do not use this in production.
>
> — [avi.im/blag/2021/fast-sqlite-inserts/](https://avi.im/blag/2021/fast-sqlite-inserts/)

</details>
