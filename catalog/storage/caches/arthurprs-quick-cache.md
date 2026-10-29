# quick_cache (S3-FIFO concurrent cache)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Storage](../README.md) › [In-memory caches](README.md)</sub>

**Tier C** (first-party numbers) · ✅ Holds up · Open source · `Rust` · `MIT` · 🟢 active

> Sharded S3-FIFO cache: up to 5.5x the hit ratio of Mutex<LRU> at similar per-op cost, about 4x faster than Mini Moka.

| | |
|---|---|
| **Beats** | An LRU cache behind std::sync::Mutex (hashlink LruCache), and Mini Moka |
| **Claimed result** | ARC S3 trace, 100000 entries: hit ratio 12.841% vs 2.327% for LRU+Mutex; run time 2.196 s vs 2.422 s single client, 0.798 s with 6 clients; Mini Moka 9.399 s. ARC DS1, 1000000 entries: 14.864% vs 3.086% hit ratio, 9.883 s vs 8.901 s (slightly slower single-threaded). |
| **Workload** | mokabench replaying ARC paper traces (S3 search, DS1 database, SPC1) with 1, 3 and 6 client threads; a miss triggers an insert; no simulated miss penalty. |
| **Metric** | hit ratio and time |
| **Hardware** | Intel i9-12900H |
| **OS** | x64 Linux |
| **Measured** | 2024-09-01 |
| **Adoption** | 363 stars |
| **Last commit** | 2026-06-26 |
| **Platforms** | Any Rust target; no background threads |
| **Install** | cargo add quick_cache |
| **Links** | [repository](https://github.com/arthurprs/quick-cache) · [documentation](https://docs.rs/quick_cache) · [benchmark](https://github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063ecfef6a717b/README.md) |

## Verdict

On the ARC traces it reaches 3x to 5.5x the hit ratio of a mutex-wrapped LRU at small capacities while costing about the same per operation single-threaded, and it scales with threads. It ran 2.6x to 4.3x faster than Mini Moka at similar hit ratios.

**Why tier C:** First-party run of the public mokabench harness with the CPU named, but no rustc version or flags.

## Use it when

- A hand-rolled Mutex<LRU> or the lru crate shows a low hit ratio on scan-heavy or frequency-skewed access
- Many threads share one cache

## Skip it when

- You need time-to-live, time-to-idle or per-entry expiry: moka has them, quick_cache does not
- Single-threaded code with a cache that already holds the working set: a plain LRU was as fast

## How it is fast

S3-FIFO keeps a small probationary queue and a main queue with a ghost list, so one-hit items are evicted early without the list reordering LRU needs on every hit. Shards with their own locks spread contention across threads, and there are no background maintenance threads.

## What it costs

- Fewer features than moka (no TTL, no eviction listener beyond lifecycle hooks)
- Approximate eviction order per shard rather than strict global LRU

## Problems with the evidence

- First-party run, September 2024, quick_cache 0.6; the crate is now 0.7
- mokabench charges nothing for a miss, so hit-ratio gains show up only as fewer inserts, not as saved backend latency

## Evidence

<details>
<summary>10 quotes from the sources, checked 2026-10-28</summary>

> Benchmarks performed with [mokabench](https://github.com/moka-rs/mokabench) in a x64 Linux OS + Intel i9-12900H CPU.
>
> — [github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063…](https://github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063ecfef6a717b/README.md)

> ### Trace 1 (S3 from the Arc paper)
>
> — [github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063…](https://github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063ecfef6a717b/README.md)

> ### Trace 2 (DS1 from the Arc paper)
>
> — [github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063…](https://github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063ecfef6a717b/README.md)

> | **QuickCache** | 100000 | 1 | 14300769 | 16407702 | 12.841 | 2.196 |
>
> — [github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063…](https://github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063ecfef6a717b/README.md)

> | **QuickCache** | 100000 | 6 | 14300809 | 16407702 | 12.841 | 0.798 |
>
> — [github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063…](https://github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063ecfef6a717b/README.md)

> | LRU+Mutex | 100000 | 1 | 16025830 | 16407702 | 2.327 | 2.422 |
>
> — [github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063…](https://github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063ecfef6a717b/README.md)

> | Mini Moka | 100000 | 1 | 14695340 | 16407702 | 10.436 | 9.399 |
>
> — [github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063…](https://github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063ecfef6a717b/README.md)

> | **QuickCache** | 1000000 | 1 | 37208683 | 43704979 | 14.864 | 9.883 |
>
> — [github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063…](https://github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063ecfef6a717b/README.md)

> | LRU+Mutex | 1000000 | 1 | 42356290 | 43704979 | 3.086 | 8.901 |
>
> — [github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063…](https://github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063ecfef6a717b/README.md)

> * LRU+Mutex: hashlink crate + std::sync::Mutex
>
> — [github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063…](https://github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063ecfef6a717b/README.md)

</details>
