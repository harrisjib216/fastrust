# DashMap (sharded RwLock hash map)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Concurrent maps](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Open source · `Rust` · `MIT` · 🟢 active

> Sharded concurrent HashMap with a HashMap-like &self API; scales far past RwLock<HashMap> under contention but is slower single-threaded.

| | |
|---|---|
| **Beats** | RwLock<HashMap> (parking_lot RwLock around std HashMap in the measurements) |
| **Claimed result** | Read-heavy, FxHash, Ryzen 9 3950X: 87.5 vs 11.7 Mops/s at 16 threads (about 7.5x, derived from page data) but 14.1 vs 19.4 at 1 thread; Apple M2 Max: 42.2 vs 3.4 at 12 threads (about 12x, derived from page data), 10.4 vs 21.2 at 1 thread. Exchange mix with SipHash at 8 threads: 26233769.293655325 vs 1331625.593635424 ops/s (about 20x, derived from page data). |
| **Workload** | bustle harness (port of the libcuckoo benchmark) with u64 keys: read-heavy (98% reads, 1% insert, 1% remove) and exchange (10% read, 40% insert, 40% remove, 10% update). |
| **Metric** | throughput (operations per second) |
| **Hardware** | AMD Ryzen 9 3950X (16 cores); Apple M2 Max (12 cores) |
| **OS** | Ubuntu 20.04 (3950X) |
| **Toolchain** | nightly Rust (2022), lto = true, codegen-units = 1; dashmap 5.0 |
| **Measured** | 2022-03 |
| **Adoption** | 4.1k stars |
| **Last commit** | 2026-10-21 |
| **Platforms** | All targets with std |
| **Install** | cargo add dashmap |
| **Links** | [repository](https://github.com/xacrimon/dashmap) · [documentation](https://docs.rs/dashmap) · [benchmark](https://github.com/robclu/leapfrog/blob/main/README.md) |

## Verdict

Under contention DashMap beats a single-lock map by large factors: 7.5x on a read-heavy mix at 16 threads and about 20x on an insert/remove-heavy mix at 8 threads. On one thread it is slower than RwLock<HashMap> (0.72x and 0.49x on two CPUs).

**Why tier A:** Independent results (leapfrog's fork of conc-map-bench) with public code, CPU and OS, and LTO/codegen-units flags stated.

## Use it when

- Many threads read and write a shared map and a single RwLock<HashMap> shows contention.
- Write-heavy or mixed workloads, where its sharded locks held up far better than flurry and evmap in the same runs.

## Skip it when

- Access is mostly single-threaded: RwLock<HashMap> was faster on one thread.
- You hold a Ref or RefMut across other map calls or .await points: a second lock on the same shard deadlocks.

## How it is fast

DashMap splits the table into many shards, each a hashbrown table behind its own RwLock, chosen by key hash. Threads touching different shards do not contend, so lock cache-line traffic is spread out instead of concentrated on one lock.

## What it costs

- Readers still take shard locks, so a slow writer blocks readers of that shard.
- Iteration locks shards one at a time and is not a consistent snapshot.

## Problems with the evidence

- Measured with dashmap 5.0 in 2022; current releases are 6.x.
- The baseline used parking_lot's RwLock, not std's; std's RwLock moved to futexes on Linux in Rust 1.62 (2022), after this run.
- The read-heavy table used FxHash for every map; the exchange numbers use SipHash.
- dashmap's own README points to conc-map-bench, which is run by dashmap's author and whose 2024 results omit an RwLock<HashMap> baseline.

## Evidence

<details>
<summary>9 quotes from the sources, checked 2026-10-28</summary>

> | RwLock + HashMap | 19.4           | 1.0          | 11.7            | 0.60          |
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

> | DashMap          | 14.1           | 0.72         | 87.5            | 4.51          |
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

> | RwLock + HashMap | 21.2           | 1.0          | 3.4             | 0.16          |
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

> | DashMap          | 10.4           | 0.49         | 42.2            | 2.00          |
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

> RwLock<StdHashMap>,8388608,8,6299524461,1331625.593635424,6007
>
> — [github.com/robclu/conc-map-bench/blob/master/results/Exchange.std.csv](https://github.com/robclu/conc-map-bench/blob/master/results/Exchange.std.csv)

> DashMap,8388608,8,319763733,26233769.293655325,304
>
> — [github.com/robclu/conc-map-bench/blob/master/results/Exchange.std.csv](https://github.com/robclu/conc-map-bench/blob/master/results/Exchange.std.csv)

> CPU: AMD Threadripper 3950x
>
> — [github.com/robclu/conc-map-bench/blob/master/README.md](https://github.com/robclu/conc-map-bench/blob/master/README.md)

> | Map              | Throughput (1) | Relative (1) | Throughput (12) | Relative (12) |
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

> A snapshot of the results for the LeapMap on a read heavy workload using a 16 core AMD 3950x CPU
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

</details>
