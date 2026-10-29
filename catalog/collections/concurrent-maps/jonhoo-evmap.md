# evmap (left-right eventually consistent map)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Concurrent maps](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Eventually consistent multi-value map built on left-right; lock-free reads that scale, at the cost of slow, batched writes.

| | |
|---|---|
| **Beats** | RwLock<HashMap> (parking_lot RwLock around std HashMap in the measurements) |
| **Claimed result** | Read-heavy, Apple M2 Max: 18.7 vs 3.4 Mops/s at 12 threads (about 5.5x, derived from page data), 13.5 vs 21.2 at 1 thread. Exchange mix with SipHash on Ryzen 9 3950X: 701044.6231179577 ops/s at 16 threads vs 1331625.593635424 for RwLock<HashMap> at 8 threads. |
| **Workload** | bustle harness with u64 keys: read-heavy (98% reads) and exchange (40% insert, 40% remove); the adapter takes a Mutex on the write handle and refreshes after every write. |
| **Metric** | throughput (operations per second) |
| **Hardware** | Apple M2 Max (12 cores); AMD Ryzen 9 3950X (16 cores) |
| **OS** | Ubuntu 20.04 (3950X) |
| **Toolchain** | nightly Rust (2022), lto = true, codegen-units = 1; evmap 10.0.2 |
| **Measured** | 2023-06 |
| **Adoption** | 580 stars |
| **Last commit** | 2026-08-02 |
| **Platforms** | All targets with std |
| **Install** | cargo add evmap |
| **Links** | [repository](https://github.com/jonhoo/evmap) · [documentation](https://docs.rs/evmap) · [benchmark](https://github.com/robclu/leapfrog/blob/main/README.md) |

## Verdict

With many readers, evmap reads scale past a single-lock map (about 5.5x at 12 threads on an M2 Max). When every write is published immediately it is the slowest map tested on an insert/remove-heavy mix, below RwLock<HashMap>, and on one thread it is slower too. The win needs read-mostly access and writers that batch refreshes.

**Why tier A:** Independent results (leapfrog's fork of conc-map-bench) with public code, CPU, OS and flags stated; evmap's own numbers are charts it calls outdated.

## Use it when

- Read-mostly maps (configuration, routing, caches) where readers can tolerate seeing writes only after the writer refreshes.
- Writers can batch many changes per refresh.

## Skip it when

- Writes are frequent and must be visible immediately: refreshing per write made it the slowest map on the exchange mix.
- Memory is tight: it stores two copies of the data plus a per-key Vec for multiple values.
- Multiple concurrent writers: writes need a Mutex around the single write handle.

## How it is fast

evmap keeps two copies of the map. Readers read one copy with no locks; the single writer applies changes to the other copy and swaps them on refresh, then replays the operation log. Reads never contend with the writer, but each write is applied twice.

## What it costs

- Eventual consistency: readers see only what preceded the last refresh.
- Double memory and double write work.

## Problems with the evidence

- The bustle adapter refreshes after every write, evmap's documented worst case.
- Measured evmap 10.0.2; the current release is 11.0.0.
- Baseline was parking_lot's RwLock, measured before std's RwLock moved to futexes on Linux in Rust 1.62.
- No RwLock<HashMap> result at 16 threads was recorded for the exchange mix; the comparison uses its 8-thread figure.

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> | Evmap            | 13.5           | 0.64         | 18.7            | 0.88          |
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

> | RwLock + HashMap | 21.2           | 1.0          | 3.4             | 0.16          |
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

> Evmap,8388608,16,11965868824,701044.6231179577,22823
>
> — [github.com/robclu/conc-map-bench/blob/master/results/Exchange.std.csv](https://github.com/robclu/conc-map-bench/blob/master/results/Exchange.std.csv)

> RwLock<StdHashMap>,8388608,8,6299524461,1331625.593635424,6007
>
> — [github.com/robclu/conc-map-bench/blob/master/results/Exchange.std.csv](https://github.com/robclu/conc-map-bench/blob/master/results/Exchange.std.csv)

> This benchmark represents the worst-case usage of
>
> — [github.com/jonhoo/evmap/blob/main/README.md](https://github.com/jonhoo/evmap/blob/main/README.md)

> | Map              | Throughput (1) | Relative (1) | Throughput (12) | Relative (12) |
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

> A snapshot of the results for the LeapMap on a read heavy workload using a 16 core AMD 3950x CPU
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

</details>
