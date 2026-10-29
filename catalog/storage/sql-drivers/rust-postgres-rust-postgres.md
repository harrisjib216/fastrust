# tokio-postgres (async PostgreSQL driver)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Storage](../README.md) › [SQL drivers and ORMs](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Async PostgreSQL driver; 1.5x to 1.9x faster than sqlx on 10,000-row results, slower on one-row queries in the same suite.

| | |
|---|---|
| **Beats** | sqlx 0.8 PostgreSQL driver (query_as! macro and query + FromRow) |
| **Claimed result** | Trivial select, median: 10000 rows 2,190.77 us vs 3,347.92 us (query_as!) and 4,133.86 us (FromRow), 1.53x and 1.89x; 1 row 53.45 us vs 34.32 us (sqlx faster). User report fetching 12096001 rows: 9386ms vs 29830ms for sqlx query_as (ratios derived from page data). |
| **Workload** | Diesel benchmark 'trivial query' against a local PostgreSQL 18: SELECT returning 1 to 10,000 user rows into structs; a separate user report fetched 12 million 6-column rows. |
| **Metric** | latency |
| **Hardware** | GitHub Actions ubuntu-latest runner (shared VM, CPU not recorded) |
| **OS** | Ubuntu (GitHub Actions ubuntu-latest image), PostgreSQL 18 on localhost |
| **Toolchain** | rustc stable via rustup on the run date, RUSTFLAGS=-C target-cpu=native, cargo bench |
| **Measured** | 2026-10-27 |
| **Adoption** | 4.0k stars |
| **Last commit** | 2026-10-03 |
| **Platforms** | Any tokio-supported target |
| **Install** | cargo add tokio-postgres |
| **Links** | [repository](https://github.com/rust-postgres/rust-postgres) · [documentation](https://docs.rs/tokio-postgres) · [benchmark](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html) |

## Verdict

tokio-postgres loads large result sets 1.5x faster than sqlx's macro path and 1.9x faster than its FromRow path at 10,000 rows. On one-row queries in the same suite it was 1.56x slower than sqlx, so the win depends on result size.

**Why tier A:** Public suite run nightly in CI by the Diesel project, independent of both drivers, with toolchain and RUSTFLAGS stated.

## Use it when

- Endpoints that return thousands of rows per query
- You can pipeline independent queries, which sqlx does not support

## Skip it when

- Mostly single-row lookups, where sqlx was faster in this suite
- You want compile-time checked SQL (sqlx macros) or a query builder (Diesel)

## How it is fast

tokio-postgres decodes rows lazily from the binary protocol buffer and supports pipelining several queries on one connection. sqlx adds its own row and type-mapping layer, which costs more as the row count grows.

## What it costs

- No connection pool, migrations or compile-time query checks built in
- Row-to-struct mapping is manual or needs a helper crate

## Problems with the evidence

- Shared CI VM; numbers change run to run (this record uses the 2026-10-27 run)
- Suite pins sqlx 0.8; sqlx 0.9 (May 2026) is not measured
- The one-row result may reflect per-iteration runtime overhead in the harness: the synchronous postgres crate, which wraps tokio-postgres, took 30.22 us
- The 12M-row report ran tokio-postgres first, so database caching may have favored the second library less than it seems

## Evidence

<details>
<summary>16 quotes from the sources, checked 2026-10-28</summary>

> Trivial query (postgres)
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 2026-10-27 04:43:23 (dde3b22cae58b9f0fd0e7f9e9a2b3dd62377baec)
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 10000
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 2,190.77
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 3,347.92
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 4,133.86
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 53.45
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 34.32
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 30.22
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> RUSTFLAGS: "-C target-cpu=native"
>
> — [github.com/diesel-rs/diesel/blob/82bd9b322bcc7f4e3f9324785cda8e8735…](https://github.com/diesel-rs/diesel/blob/82bd9b322bcc7f4e3f9324785cda8e87351b0171/.github/workflows/metrics.yml)

> about 12096001 rows x 6 columns
>
> — [github.com/transact-rs/sqlx/issues/2007](https://github.com/transact-rs/sqlx/issues/2007)

> tokio_postgres_query_time: 9386ms
>
> — [github.com/transact-rs/sqlx/issues/2007](https://github.com/transact-rs/sqlx/issues/2007)

> sqlx_query_time query_as: 29830ms
>
> — [github.com/transact-rs/sqlx/issues/2007](https://github.com/transact-rs/sqlx/issues/2007)

> This is with tokio-postgres running the query first, sqlx second, so the db might cache something already for sqlx?
>
> — [github.com/transact-rs/sqlx/issues/2007](https://github.com/transact-rs/sqlx/issues/2007)

> Query Pipelining is supported by diesel-async and tokio-postgres
>
> — [diesel.rs/compare_diesel.html](https://diesel.rs/compare_diesel.html)

> Notably sqlx and sea-orm do not support this feature.
>
> — [diesel.rs/compare_diesel.html](https://diesel.rs/compare_diesel.html)

</details>
