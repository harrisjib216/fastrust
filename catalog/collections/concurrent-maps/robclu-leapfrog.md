# leapfrog (LeapMap concurrent hash map)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Concurrent maps](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Lock-free concurrent map using leapfrog probing; about 12x RwLock<HashMap> throughput at 16 threads on reads, with value-type restrictions.

| | |
|---|---|
| **Beats** | RwLock<HashMap> (parking_lot RwLock around std HashMap) |
| **Claimed result** | Claimed: around 13.5x faster than RwLock<HashMap> on 16 cores. Table: 148.0 vs 11.7 Mops/s at 16 threads on Ryzen 9 3950X (about 12.6x, derived from page data), 17.8 vs 19.4 at 1 thread; Apple M2 Max: 106.1 vs 3.4 at 12 threads. |
| **Workload** | bustle read-heavy mix (98% reads, 1% insert, 1% remove) with u64 keys and FxHash. |
| **Metric** | throughput (Mops/s) |
| **Hardware** | AMD Ryzen 9 3950X (16 cores); Apple M2 Max (12 cores) |
| **OS** | Ubuntu 20.04 (3950X) |
| **Toolchain** | nightly Rust (2022), lto = true, codegen-units = 1 |
| **Measured** | 2022-03 |
| **Adoption** | 266 stars |
| **Last commit** | 2026-03-23 |
| **Platforms** | All targets with std; needs nightly allocator_api or the stable_alloc feature |
| **Install** | cargo add leapfrog --features stable_alloc |
| **Links** | [repository](https://github.com/robclu/leapfrog) · [documentation](https://docs.rs/leapfrog) · [benchmark](https://github.com/robclu/leapfrog/blob/main/README.md) |

## Verdict

On a read-heavy mix LeapMap reached 148.0 Mops/s at 16 threads against 11.7 for RwLock<HashMap>, about 12.6x, close to the README's 'around 13.5x', and it led DashMap and flurry. Single-threaded it is slightly slower than the locked map.

**Why tier A:** Public benchmark fork with result CSVs, CPU, OS and LTO/codegen-units flags stated; first-party.

## Use it when

- Read-heavy shared maps with small, atomic-friendly key and value types such as integers.

## Skip it when

- You cannot reserve two values of the value type: the Value trait needs a null and a redirect sentinel (MAX and MAX-1 for integers).
- Single-threaded access, where RwLock<HashMap> was slightly faster.
- You need a widely used, heavily reviewed crate: it has about 32 thousand downloads.

## How it is fast

LeapMap is an open-addressing table where each bucket stores offsets to the next probe positions (leapfrog probing), so lookups jump directly along a key's probe chain. Keys and values that fit in atomics are read and written with atomic operations and no locks; other types fall back to fine-grained spinlocks.

## What it costs

- Reserved sentinel values in the value type.
- Requires a nightly feature or the stable_alloc compatibility feature.

## Problems with the evidence

- First-party numbers; the README's 13.5x is higher than its own table's 12.6x.
- All maps in the table used FxHash; the baseline was parking_lot's RwLock, measured before std's RwLock moved to futexes on Linux in Rust 1.62.

## Evidence

<details>
<summary>8 quotes from the sources, checked 2026-10-28</summary>

> Compared to `RwLock<HashMap>`, on 16 cores the `LeapMap` is around 13.5x faster.
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

> | RwLock + HashMap | 19.4           | 1.0          | 11.7            | 0.60          |
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

> | LeapMap          | 17.8           | 0.92         | 148.0           | 7.62          |
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

> | LeapMap          | 13.9           | 0.66         | 106.1           | 5.00          |
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

> | RwLock + HashMap | 21.2           | 1.0          | 3.4             | 0.16          |
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

> OS: Ubuntu 20.4
>
> — [github.com/robclu/conc-map-bench/blob/master/README.md](https://github.com/robclu/conc-map-bench/blob/master/README.md)

> | Map              | Throughput (1) | Relative (1) | Throughput (12) | Relative (12) |
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

> A snapshot of the results for the LeapMap on a read heavy workload using a 16 core AMD 3950x CPU
>
> — [github.com/robclu/leapfrog/blob/main/README.md](https://github.com/robclu/leapfrog/blob/main/README.md)

</details>
