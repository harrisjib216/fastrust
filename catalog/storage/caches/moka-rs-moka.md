# moka (TinyLFU concurrent cache, with Mini Moka)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Storage](../README.md) › [In-memory caches](README.md)</sub>

**Tier B** (independent measurement) · 🟡 Holds up in part · Open source · `Rust` · `(MIT OR Apache-2.0) AND Apache-2.0` · 🟢 active

> Caffeine-style cache with TTL and listeners; 4.5x the hit ratio of Mutex<LRU> on a search trace, but 7.3x the per-op cost.

| | |
|---|---|
| **Beats** | An LRU cache behind std::sync::Mutex (hashlink LruCache) |
| **Claimed result** | ARC S3 trace, 100000 entries: hit ratio 10.439% vs 2.327% for Mutex<LRU> (4.5x), but run time 17.728 s vs 2.422 s (7.3x slower); Mini Moka 9.399 s (3.9x slower). ARC DS1 trace, 1000000 entries: 14.907% vs 3.086% hit ratio, 74.586 s vs 8.901 s (ratios derived from page data). |
| **Workload** | mokabench replaying ARC paper traces with 1, 3 and 6 client threads; a miss triggers an insert; no simulated miss penalty. |
| **Metric** | hit ratio and time |
| **Hardware** | Intel i9-12900H |
| **OS** | x64 Linux |
| **Measured** | 2024-09-01 |
| **Adoption** | 2.7k stars |
| **Last commit** | 2026-08-09 |
| **Platforms** | Any Rust target with std; sync and async (future) caches |
| **Install** | cargo add moka --features sync |
| **Links** | [repository](https://github.com/moka-rs/moka) · [documentation](https://docs.rs/moka) · [benchmark](https://github.com/arthurprs/quick-cache/issues/49) |

## Verdict

Moka's TinyLFU policy does raise hit ratio, 4.5x over a mutex-wrapped LRU on the S3 trace. But a third-party mokabench run found its per-operation cost 7.3x higher single-threaded (Mini Moka 3.9x). It only comes out ahead when a miss costs far more than a cache operation.

**Why tier B:** Independent run (quick_cache's author) of moka's own public mokabench harness; CPU named, rustc not stated.

## Use it when

- Misses are expensive (database, network) so hit ratio dominates total time
- You need time-to-live, time-to-idle, per-entry expiry, eviction listeners or async get_with

## Skip it when

- Cache operations themselves are the hot path and misses are cheap: a Mutex<LRU> or quick_cache was several times faster
- Small dependency trees matter

## How it is fast

A count-min frequency sketch decides whether a new entry may displace the LRU victim (TinyLFU admission), which keeps one-off keys from flushing popular ones. Reads and writes are recorded in buffers and applied in batches to a lock-free hash table.

## What it costs

- Larger dependency tree than simpler caches
- Eviction is approximate and asynchronous; size bounds are best-effort

## Problems with the evidence

- Run in September 2024 with moka 0.12 by the author of a competing crate
- Moka's own hit-ratio comparisons with LRU are published only as charts (moka v0.7.0)
- No miss penalty in mokabench, which understates the value of a higher hit ratio

## Evidence

<details>
<summary>8 quotes from the sources, checked 2026-10-28</summary>

> HashLink (LRU w/ Mutex), 100000, 1, 16025830, 16407702, 2.327, 2.422
>
> — [github.com/arthurprs/quick-cache/issues/49](https://github.com/arthurprs/quick-cache/issues/49)

> trace_file: S3
>
> — [github.com/arthurprs/quick-cache/issues/49](https://github.com/arthurprs/quick-cache/issues/49)

> trace_file: Ds1
>
> — [github.com/arthurprs/quick-cache/issues/49](https://github.com/arthurprs/quick-cache/issues/49)

> Moka Sync Cache, 100000, 1, 14694850, 16407702, 10.439, 17.728
>
> — [github.com/arthurprs/quick-cache/issues/49](https://github.com/arthurprs/quick-cache/issues/49)

> Mini Moka Sync Cache, 100000, 1, 14695340, 16407702, 10.436, 9.399
>
> — [github.com/arthurprs/quick-cache/issues/49](https://github.com/arthurprs/quick-cache/issues/49)

> HashLink (LRU w/ Mutex), 1000000, 1, 42356290, 43704979, 3.086, 8.901
>
> — [github.com/arthurprs/quick-cache/issues/49](https://github.com/arthurprs/quick-cache/issues/49)

> Moka Sync Cache, 1000000, 1, 37189718, 43704979, 14.907, 74.586
>
> — [github.com/arthurprs/quick-cache/issues/49](https://github.com/arthurprs/quick-cache/issues/49)

> Benchmarks performed with [mokabench](https://github.com/moka-rs/mokabench) in a x64 Linux OS + Intel i9-12900H CPU.
>
> — [github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063…](https://github.com/arthurprs/quick-cache/blob/c5f89f146be06a0a9720afd520063ecfef6a717b/README.md)

</details>
