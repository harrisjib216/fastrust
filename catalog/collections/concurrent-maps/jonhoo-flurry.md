# flurry (port of Java's ConcurrentHashMap)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Concurrent maps](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟡 stale

> Rust port of Java's ConcurrentHashMap; scales on reads but has memory and write-throughput problems its author acknowledges.

| | |
|---|---|
| **Beats** | RwLock<HashMap> (parking_lot RwLock around std HashMap in the measurements) |
| **Claimed result** | Read-heavy, FxHash, Ryzen 9 3950X: 80.8 vs 11.7 Mops/s at 16 threads (about 6.9x, derived from page data), 11.2 vs 19.4 at 1 thread; Apple M2 Max: 64.5 vs 3.4 at 12 threads. Exchange mix with SipHash at 16 threads: 9627180.87311915 ops/s vs 40457181.8270502 for DashMap. |
| **Workload** | bustle harness with u64 keys: read-heavy (98% reads) and exchange (10% read, 40% insert, 40% remove, 10% update). |
| **Metric** | throughput (operations per second) |
| **Hardware** | AMD Ryzen 9 3950X (16 cores); Apple M2 Max (12 cores) |
| **OS** | Ubuntu 20.04 (3950X) |
| **Toolchain** | nightly Rust (2022), lto = true, codegen-units = 1; flurry 0.3.1 |
| **Measured** | 2022-03 |
| **Adoption** | 580 stars |
| **Last commit** | 2024-11-19 |
| **Platforms** | All targets with std |
| **Install** | cargo add flurry |
| **Links** | [repository](https://github.com/jonhoo/flurry) · [documentation](https://docs.rs/flurry) · [benchmark](https://github.com/robclu/leapfrog/blob/main/README.md) |

## Verdict

flurry beats a single RwLock<HashMap> on a read-heavy mix at high thread counts (about 6.9x at 16 threads), but is slower on one thread, about 4x behind DashMap on the insert/remove-heavy mix, and was killed for running out of memory in one run. Its own README now steers users to papaya or dashmap.

**Why tier A:** Independent results (leapfrog's fork of conc-map-bench) with public code, CPU, OS and build flags stated.

## Use it when

- Rarely: read-mostly maps where you specifically want Java ConcurrentHashMap semantics.

## Skip it when

- Most new code: the author recommends papaya or dashmap.
- Insert/remove-heavy workloads, where it trailed DashMap by about 4x and one run ran out of memory.
- Single-threaded or lightly contended access, where RwLock<HashMap> was faster.

## How it is fast

Reads are lock-free: bins are reached through atomic pointers and memory is reclaimed with epoch-based garbage collection, so readers never block. Writers lock only the bin they modify, and resizing is cooperative across threads.

## What it costs

- Epoch-based reclamation delays freeing memory and raised memory use under load.
- Every value access goes through a guard.

## Problems with the evidence

- Measured flurry 0.3.1 in 2022; the current release is 0.5.2.
- Baseline was parking_lot's RwLock, before std's RwLock moved to futexes on Linux in Rust 1.62.

## Evidence

<details>
<summary>10 quotes from the sources, checked 2026-10-28</summary>

> | RwLock + HashMap | 19.4           | 1.0          | 11.7            | 0.60          |
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

> | Flurry           | 11.2           | 0.58         | 80.8            | 4.16          |
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

> | Flurry           | 8.0            | 0.37         | 64.5            | 3.04          |
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

> | RwLock + HashMap | 21.2           | 1.0          | 3.4             | 0.16          |
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

> Flurry,8388608,16,871346255,9627180.87311915,1661
>
> — [github.com/robclu/conc-map-bench/blob/master/results/Exchange.std.csv](https://github.com/robclu/conc-map-bench/blob/master/results/Exchange.std.csv)

> DashMap,8388608,16,207345337,40457181.8270502,395
>
> — [github.com/robclu/conc-map-bench/blob/master/results/Exchange.std.csv](https://github.com/robclu/conc-map-bench/blob/master/results/Exchange.std.csv)

> the `flurry` case fails because of the OOM killer.
>
> — [github.com/robclu/conc-map-bench/blob/master/README.md](https://github.com/robclu/conc-map-bench/blob/master/README.md)

> Flurry currently suffers performance and memory usage issues under load.
>
> — [github.com/jonhoo/flurry/blob/main/README.md](https://github.com/jonhoo/flurry/blob/main/README.md)

> | Map              | Throughput (1) | Relative (1) | Throughput (12) | Relative (12) |
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

> A snapshot of the results for the LeapMap on a read heavy workload using a 16 core AMD 3950x CPU
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

</details>
