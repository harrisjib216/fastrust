# zero-postgres (sync and async PostgreSQL client)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Storage](../README.md) › [SQL drivers and ORMs](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Open source · `Rust` · `MIT` · 🟢 active

> New PostgreSQL client that tops Diesel's select benchmarks: 1.3x faster than tokio-postgres, about 2x sqlx on 10,000 rows.

| | |
|---|---|
| **Beats** | tokio-postgres, sqlx and Diesel on PostgreSQL |
| **Claimed result** | README: 10,000 prepared inserts 250.35 ms vs 398.90 ms for tokio-postgres (1.59x). Diesel suite, trivial select 10000 rows median: 1,699.58 us vs 2,190.77 us tokio-postgres, 2,419.35 us Diesel, 3,347.92 us sqlx; 100-row insert 505.93 us vs 551.50 us Diesel and 1,096.32 us sqlx (ratios derived from page data). |
| **Workload** | Inserting 10,000 rows with prepared statements (README); Diesel suite trivial SELECT returning 1 to 10,000 rows and batch INSERT of 1 to 100 rows against local PostgreSQL 18. |
| **Metric** | latency |
| **Hardware** | GitHub Actions ubuntu-latest runner (Diesel suite); not stated for the README run |
| **OS** | Ubuntu (GitHub Actions ubuntu-latest image), PostgreSQL 18 on localhost |
| **Toolchain** | rustc stable via rustup on the run date, RUSTFLAGS=-C target-cpu=native, cargo bench (Diesel suite) |
| **Measured** | 2026-10-27 |
| **Adoption** | 11 stars |
| **Last commit** | 2026-10-15 |
| **Platforms** | Sync API, tokio and (experimental) compio runtimes |
| **Install** | cargo add zero-postgres |
| **Links** | [repository](https://github.com/elbaro/zero-postgres) · [documentation](https://docs.rs/zero-postgres) · [benchmark](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html) |

## Verdict

Its README claims 1.59x faster prepared inserts than tokio-postgres. Diesel's independent nightly suite ranks it fastest in every PostgreSQL select group (1.29x ahead of tokio-postgres and about 2x ahead of sqlx on 10,000-row selects) and within 6% of the fastest in every insert group.

**Why tier A:** Independent public suite (Diesel's nightly CI) with toolchain and RUSTFLAGS stated confirms the first-party README figure.

## Use it when

- Postgres row loading or bulk inserts are a measured bottleneck and you can adopt a young library

## Skip it when

- You need a mature ecosystem, pooling integrations and a long track record
- You need production TLS today: TLS support is marked experimental

## How it is fast

The name and the RefFromRow derive point to zero-copy row decoding, borrowing column data from the receive buffer instead of allocating per field. The README does not document its internals in detail.

## What it costs

- Young API that may change before 1.0
- The repository now states BlueOak-1.0.0 while the published 0.9.0 crate says MIT

## Problems with the evidence

- Very new: first release December 2025, 11 GitHub stars; Diesel's suite pins version 0.8.0 while 0.9.0 is current
- README benchmark gives no hardware or toolchain
- Diesel-suite numbers come from a shared CI VM

## Evidence

<details>
<summary>15 quotes from the sources, checked 2026-10-28</summary>

> Inserting 10,000 rows using prepared statements (average of 10 iterations):
>
> — [github.com/elbaro/zero-postgres/blob/dff70d77bfa7f6a1df3390a5d0e962…](https://github.com/elbaro/zero-postgres/blob/dff70d77bfa7f6a1df3390a5d0e9624108f1171a/README.md)

> | zero-postgres (sync) | 250.35 |
>
> — [github.com/elbaro/zero-postgres/blob/dff70d77bfa7f6a1df3390a5d0e962…](https://github.com/elbaro/zero-postgres/blob/dff70d77bfa7f6a1df3390a5d0e9624108f1171a/README.md)

> | tokio-postgres | 398.90 |
>
> — [github.com/elbaro/zero-postgres/blob/dff70d77bfa7f6a1df3390a5d0e962…](https://github.com/elbaro/zero-postgres/blob/dff70d77bfa7f6a1df3390a5d0e9624108f1171a/README.md)

> Trivial query (postgres)
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 10000
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 1,699.58
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 2,190.77
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 2,419.35
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> 3,347.92
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_trivial_query_postgres.html)

> Insert (postgres)
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_insert_postgres.html)

> 100
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_insert_postgres.html)

> 505.93
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_insert_postgres.html)

> 551.50
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_insert_postgres.html)

> 1,096.32
>
> — [github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff217…](https://github.com/diesel-rs/metrics/blob/4f1da2d29edd06e9507d864bb508ff21717589f6/plots/table_bench_insert_postgres.html)

> RUSTFLAGS: "-C target-cpu=native"
>
> — [github.com/diesel-rs/diesel/blob/82bd9b322bcc7f4e3f9324785cda8e8735…](https://github.com/diesel-rs/diesel/blob/82bd9b322bcc7f4e3f9324785cda8e87351b0171/.github/workflows/metrics.yml)

</details>
