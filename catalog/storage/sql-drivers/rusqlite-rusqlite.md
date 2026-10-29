# rusqlite (synchronous SQLite driver)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Storage](../README.md) › [SQL drivers and ORMs](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Open source · `Rust + C` · `MIT` · 🟢 active

> Synchronous SQLite bindings; queries ran 11x to 49x faster than sqlx's async SQLite driver in Diesel's nightly suite.

| | |
|---|---|
| **Beats** | sqlx 0.8 async SQLite driver (query_as! macro and query + FromRow) |
| **Claimed result** | Trivial select, median: 1 row 0.47 us vs 22.86 us for sqlx query_as! (sqlx 49.09x slower); 10000 rows 1,897.01 us vs 21,446.58 us (11.31x). Separate bulk-insert test with a naive insert loop: rusqlite about 3 minutes vs sqlx about 14 mins. |
| **Workload** | Diesel benchmark 'trivial query': SELECT from a users table returning 1 to 10,000 rows into structs, local SQLite file; separately, 100M single-row inserts. |
| **Metric** | latency |
| **Hardware** | GitHub Actions ubuntu-latest runner (shared VM, CPU not recorded) |
| **OS** | Ubuntu (GitHub Actions ubuntu-latest image) |
| **Toolchain** | rustc stable via rustup on the run date, RUSTFLAGS=-C target-cpu=native, cargo bench |
| **Measured** | 2026-10-28 |
| **Adoption** | 4.5k stars |
| **Last commit** | 2026-10-27 |
| **Platforms** | Any target SQLite builds on (bundled feature compiles SQLite from source) |
| **Install** | cargo add rusqlite |
| **Links** | [repository](https://github.com/rusqlite/rusqlite) · [documentation](https://docs.rs/rusqlite) · [benchmark](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_sqlite.html) |

## Verdict

Diesel's nightly suite measures rusqlite 49x faster than sqlx on a one-row query and 11x faster on 10,000 rows. A separate 2021 insert test found the same direction (sqlx about 14 minutes vs rusqlite about 3). sqlx's SQLite driver runs SQLite on a worker thread behind channels, and that overhead dominates.

**Why tier A:** Public benchmark suite run nightly in CI by a third party (the Diesel project), with toolchain and RUSTFLAGS stated in the workflow.

## Use it when

- An application uses SQLite only; call rusqlite from spawn_blocking or a small connection pool if the rest is async
- Per-query latency or row throughput on SQLite matters

## Skip it when

- You need one async API across Postgres, MySQL and SQLite and SQLite load is light
- You rely on sqlx's compile-time checked queries for SQLite

## How it is fast

rusqlite calls the SQLite C API directly on the caller's thread. sqlx wraps SQLite, which has no async API, in a background worker and passes every statement and row over channels, adding a fixed cost per query and per row.

## What it costs

- Blocking calls: in async code they must run on a blocking thread
- SQL strings are checked at runtime, not compile time

## Problems with the evidence

- Numbers come from a shared CI VM and change run to run; this record uses the run of 2026-10-28
- The Diesel project maintains the suite; neither rusqlite nor sqlx authors ran it
- The 2021 insert test used sqlx 0.5 and rusqlite 0.25; Diesel's suite pins rusqlite 0.32 and sqlx 0.8, not the current 0.41 and 0.9

## Evidence

<details>
<summary>14 quotes from the sources, checked 2026-10-28</summary>

> Trivial query (sqlite)
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_sqlite.html)

> 2026-10-28 04:01:07 (dde3b22cae58b9f0fd0e7f9e9a2b3dd62377baec)
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_sqlite.html)

> 0.47
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_sqlite.html)

> 22.86
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_sqlite.html)

> 49.09
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_sqlite.html)

> 10000
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_sqlite.html)

> 1,897.01
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_sqlite.html)

> 21,446.58
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_sqlite.html)

> 11.31
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_sqlite.html)

> RUSTFLAGS: "-C target-cpu=native"
>
> — [github.com/diesel-rs/diesel/blob/82bd9b322bcc7f4e3f9324785cda8e8735…](https://github.com/diesel-rs/diesel/blob/82bd9b322bcc7f4e3f9324785cda8e87351b0171/.github/workflows/metrics.yml)

> runs-on: ubuntu-latest
>
> — [github.com/diesel-rs/diesel/blob/82bd9b322bcc7f4e3f9324785cda8e8735…](https://github.com/diesel-rs/diesel/blob/82bd9b322bcc7f4e3f9324785cda8e87351b0171/.github/workflows/metrics.yml)

> All async SQLite crates perform considerably worse than their sync counterparts.
>
> — [diesel.rs/compare_diesel.html](https://diesel.rs/compare_diesel.html)

> This version took about 14 mins.
>
> — [avi.im/blag/2021/fast-sqlite-inserts/](https://avi.im/blag/2021/fast-sqlite-inserts/)

> This version took about 3 minutes.
>
> — [avi.im/blag/2021/fast-sqlite-inserts/](https://avi.im/blag/2021/fast-sqlite-inserts/)

</details>
