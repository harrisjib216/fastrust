# Diesel (sync ORM and query builder)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Storage](../README.md) › [SQL drivers and ORMs](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust + C` · `MIT OR Apache-2.0` · 🟢 active

> Rust ORM with compile-time checked queries; loads 10,000-row results 1.2x to 1.4x faster than sqlx, ties it on single rows.

| | |
|---|---|
| **Beats** | sqlx 0.8 PostgreSQL driver (query_as! macro) |
| **Claimed result** | Median: trivial select 10000 rows 2,419.35 us vs 3,347.92 us for sqlx query_as! (1.38x); medium-complex query 10000 rows 2,427.76 vs 2,982.23 us (1.23x); 1 row 36.78 vs 34.32 us (sqlx faster). tokio-postgres was faster than Diesel at 10000 rows (2,190.77 us) (ratios derived from page data). |
| **Workload** | Diesel benchmark suite against local PostgreSQL 18: a trivial SELECT of 1 to 10,000 users, and a medium-complex query that left-joins users to posts filtered by hair color, mapped into structs. |
| **Metric** | latency |
| **Hardware** | GitHub Actions ubuntu-latest runner (shared VM, CPU not recorded) |
| **OS** | Ubuntu (GitHub Actions ubuntu-latest image), PostgreSQL 18 on localhost |
| **Toolchain** | rustc stable via rustup on the run date, RUSTFLAGS=-C target-cpu=native, cargo bench |
| **Measured** | 2026-10-27 |
| **Adoption** | 14.2k stars |
| **Last commit** | 2026-10-24 |
| **Platforms** | Linux, macOS, Windows; links libpq, libmysqlclient or libsqlite3 (bundling available) |
| **Install** | cargo add diesel --features postgres |
| **Links** | [repository](https://github.com/diesel-rs/diesel) · [documentation](https://docs.rs/diesel) · [benchmark](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html) |

## Verdict

Diesel's own nightly suite shows it 1.23x to 1.38x faster than sqlx's macro path at 10,000 rows, matching its claim that the gap opens on large outputs. At one row sqlx is slightly faster, and tokio-postgres and zero-postgres beat Diesel on large results, so the website's 'faster than C' line is not what the data shows.

**Why tier A:** Public suite with toolchain and RUSTFLAGS stated, run nightly in CI; first-party, but anyone can rerun the workflow.

## Use it when

- Endpoints that load hundreds to thousands of rows
- You want compile-time checked queries without a live database at build time

## Skip it when

- Single-row lookups dominate: sqlx is as fast or slightly faster
- You need the fastest raw Postgres row loading: tokio-postgres and zero-postgres were faster
- You need async I/O: diesel-async was slower than sync Diesel on small results in the same suite

## How it is fast

Diesel builds queries as types, so SQL generation and result mapping are resolved at compile time and rows deserialize straight into structs from libpq's binary results. It runs synchronously, avoiding async state machines per query.

## What it costs

- Links C client libraries by default
- Long compile times and complex type errors from the type-level query builder

## Problems with the evidence

- First-party suite on a shared CI VM; numbers change run to run
- Measured with a local database, so network round trips that dominate real deployments are absent

## Evidence

<details>
<summary>13 quotes from the sources, checked 2026-10-28</summary>

> Trivial query (postgres)
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 2026-10-27 04:43:23 (dde3b22cae58b9f0fd0e7f9e9a2b3dd62377baec)
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 10000
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 2,419.35
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 3,347.92
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 36.78
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 34.32
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 2,190.77
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 2,427.76
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_medium_complex_query_postgres.html)

> 2,982.23
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_medium_complex_query_postgres.html)

> RUSTFLAGS: "-C target-cpu=native"
>
> — [github.com/diesel-rs/diesel/blob/82bd9b322bcc7f4e3f9324785cda8e8735…](https://github.com/diesel-rs/diesel/blob/82bd9b322bcc7f4e3f9324785cda8e87351b0171/.github/workflows/metrics.yml)

> For small output sizes there is almost no difference between most of them, while the gap becomes considerably larger for large output sizes. diesel and diesel-async perform relative well here.
>
> — [diesel.rs/compare_diesel.html](https://diesel.rs/compare_diesel.html)

> Our focus on zero-cost abstractions allows Diesel to run your query and load your data even faster than C.
>
> — [diesel.rs/](https://diesel.rs/)

</details>
