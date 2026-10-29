# papaya (lock-free read-optimized hash map)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Concurrent maps](README.md)</sub>

**Tier D** (directional only) · 🟡 Holds up in part · Open source · `Rust` · `MIT` · 🟢 active

> Lock-free concurrent hash table where reads never block; aimed at read-heavy workloads and predictable latency.

| | |
|---|---|
| **Beats** | RwLock<HashMap>, and dashmap as the common concurrent alternative |
| **Claimed result** | No numbers in text. The author reports it outperforms all competitors in the read-heavy benchmark and keeps insert tail latency lower than dashmap by a few orders of magnitude. |
| **Workload** | conc-map-bench (bustle) read-heavy, exchange and rapid-grow mixes with ahash and mimalloc. |
| **Metric** | throughput; latency |
| **Hardware** | AMD Ryzen 9 9950X |
| **Measured** | 2024-11 |
| **Adoption** | 957 stars |
| **Last commit** | 2026-08-21 |
| **Platforms** | All targets with std |
| **Install** | cargo add papaya |
| **Links** | [repository](https://github.com/ibraheemdev/papaya) · [documentation](https://docs.rs/papaya) · [benchmark](https://github.com/ibraheemdev/papaya/blob/master/BENCHMARKS.md) |

## Verdict

The author reports papaya leading on read-heavy throughput and keeping insert tail latency far lower than dashmap, but publishes the results only as charts and says papaya falls short on update-heavy workloads. The claim holds, at most, for read-heavy use.

**Why tier D:** First-party conc-map-bench results published only as SVG charts and histogram images; no numbers in text.

## Use it when

- Read-dominated shared maps such as caches and routing tables.
- Async code that needs map access across .await points without lock-holding deadlocks.
- Tail latency matters more than peak write throughput.

## Skip it when

- Write-heavy workloads: the author says it falls short there and suggests alternatives.
- Memory per entry matters: every entry is a separate allocation.

## How it is fast

Every entry is allocated separately and published with atomic pointer swaps, so readers never take a lock or write shared memory. Memory is reclaimed with the seize crate, and resizing is incremental so no single operation pays for a full rehash.

## What it costs

- Allocator pressure and memory-reclamation overhead on writes.
- Access requires pinning a guard; owned guards for async are more expensive.

## Problems with the evidence

- All results are charts and images from the author.
- Benchmarks ran with mimalloc; allocator choice matters because papaya allocates per entry.

## Evidence

<details>
<summary>4 quotes from the sources, checked 2026-10-28</summary>

> As expected, it outperforms all competitors in the read-heavy benchmark.
>
> — [github.com/ibraheemdev/papaya/blob/master/BENCHMARKS.md](https://github.com/ibraheemdev/papaya/blob/master/BENCHMARKS.md)

> However, it falls short in update-heavy workloads due to allocator pressure and the overhead of memory reclamation
>
> — [github.com/ibraheemdev/papaya/blob/master/BENCHMARKS.md](https://github.com/ibraheemdev/papaya/blob/master/BENCHMARKS.md)

> keep tail latency lower by a few orders of magnitude
>
> — [github.com/ibraheemdev/papaya/blob/master/BENCHMARKS.md](https://github.com/ibraheemdev/papaya/blob/master/BENCHMARKS.md)

> All benchmarks were run on a AMD Ryzen 9 9950X processor
>
> — [github.com/ibraheemdev/papaya/blob/master/BENCHMARKS.md](https://github.com/ibraheemdev/papaya/blob/master/BENCHMARKS.md)

</details>
