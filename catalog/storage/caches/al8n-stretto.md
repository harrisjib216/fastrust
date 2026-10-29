# stretto (Rust port of Ristretto)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Storage](../README.md) › [In-memory caches](README.md)</sub>

**Tier C** (first-party numbers) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> TinyLFU cache ported from Go's Ristretto; wins hit ratio only on small caches over skewed keys, and 0.8 measured broken.

| | |
|---|---|
| **Beats** | An LRU cache behind std::sync::Mutex (hashlink LruCache), plus QuickCache and Moka |
| **Claimed result** | Current README, S3 trace at 100,000 entries: 20.72% hit ratio vs 12.83% QuickCache and 10.40% Moka; but DS1 trace at 8,000,000: 24.57% vs 69.31% QuickCache. Earlier independent mokabench run, S3 trace at 100000 entries: 0.121% hit ratio and 8.509 s vs 2.327% and 2.422 s for Mutex<LRU>. |
| **Workload** | ARC trace replay: cachebench with 16 clients (README); mokabench with 1 to 6 clients (2024 run). |
| **Metric** | hit ratio and time |
| **Hardware** | Intel i9-12900H (2024 run); not stated for the README |
| **OS** | x64 Linux (2024 run) |
| **Measured** | 2026-04 |
| **Adoption** | 435 stars |
| **Last commit** | 2026-05-03 |
| **Platforms** | Any Rust target with std; sync and async caches |
| **Install** | cargo add stretto |
| **Links** | [repository](https://github.com/al8n/stretto) · [documentation](https://docs.rs/stretto) · [benchmark](https://github.com/al8n/stretto/blob/61cdb42058b603427a3ff9672f2dc0d12407cc46/README.md) |

## Verdict

In a 2024 third-party run, stretto 0.8 reached a 0.121% hit ratio where a mutex-wrapped LRU got 2.327%, and ran 3.5x slower. Version 0.9's own README reports a fixed policy that leads only when the cache is small relative to a skewed working set and trails by up to 57 points otherwise. It gives no throughput comparison with an LRU.

**Why tier C:** Current numbers are first-party hit ratios from the author's cachebench; the only independent data covers the older 0.8 line.

## Use it when

- Working set several times larger than the cache with strongly repeated hot keys (OLTP-like traces), per the README's own decision rule

## Skip it when

- Scan-heavy or bursty traffic, or a cache sized to hold most of the data: QuickCache and Moka led there
- You need inserts to be guaranteed visible: buffered inserts can be dropped

## How it is fast

TinyLFU admission with 4-bit counters rejects new keys that are colder than the eviction victim, and sampled LFU picks victims. Inserts go through striped buffers that may drop items under contention, trading exactness for throughput.

## What it costs

- Admission policy can refuse items an LRU would keep
- Sync and async caches diverge by 5 to 20 points on some traces

## Problems with the evidence

- The independent run (September 2024) predates stretto 0.9.0 (May 2026); its author called stretto's mokabench results 'arguably broken', and no one has rerun 0.9 independently
- README benchmark lists no hardware and reports hit ratio only

## Evidence

<details>
<summary>9 quotes from the sources, checked 2026-10-28</summary>

> |   100,000 | 12.83% | 20.72% | **20.90%** |  2.80% | 10.40% | 10.49% | 10.08% |
>
> — [github.com/al8n/stretto/blob/61cdb42058b603427a3ff9672f2dc0d12407cc…](https://github.com/al8n/stretto/blob/61cdb42058b603427a3ff9672f2dc0d12407cc46/README.md)

> | 8,000,000 | **69.31%** | 24.57% | 34.64% | 53.42% | 64.38% | 64.85% | 66.43% |
>
> — [github.com/al8n/stretto/blob/61cdb42058b603427a3ff9672f2dc0d12407cc…](https://github.com/al8n/stretto/blob/61cdb42058b603427a3ff9672f2dc0d12407cc46/README.md)

> Stretto trails by up to 57 points
>
> — [github.com/al8n/stretto/blob/61cdb42058b603427a3ff9672f2dc0d12407cc…](https://github.com/al8n/stretto/blob/61cdb42058b603427a3ff9672f2dc0d12407cc46/README.md)

> trace_file: S3
>
> — [github.com/arthurprs/quick-cache/issues/49](https://github.com/arthurprs/quick-cache/issues/49)

> ### S3 trace (Search)
>
> — [github.com/al8n/stretto/blob/61cdb42058b603427a3ff9672f2dc0d12407cc…](https://github.com/al8n/stretto/blob/61cdb42058b603427a3ff9672f2dc0d12407cc46/README.md)

> ### DS1 trace (Database server)
>
> — [github.com/al8n/stretto/blob/61cdb42058b603427a3ff9672f2dc0d12407cc…](https://github.com/al8n/stretto/blob/61cdb42058b603427a3ff9672f2dc0d12407cc46/README.md)

> Stretto, 100000, 1, 16387842, 16407702, 0.121, 8.509
>
> — [github.com/arthurprs/quick-cache/issues/49](https://github.com/arthurprs/quick-cache/issues/49)

> HashLink (LRU w/ Mutex), 100000, 1, 16025830, 16407702, 2.327, 2.422
>
> — [github.com/arthurprs/quick-cache/issues/49](https://github.com/arthurprs/quick-cache/issues/49)

> It performs poorly mokabench results (arguably broken).
>
> — [github.com/arthurprs/quick-cache/issues/49](https://github.com/arthurprs/quick-cache/issues/49)

</details>
