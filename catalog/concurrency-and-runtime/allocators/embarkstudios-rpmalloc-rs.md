# rpmalloc (rpmalloc-rs global allocator)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md) › [Memory allocators](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust + C` · `MIT OR Apache-2.0` · 🟢 active

> Rust wrapper for rpmalloc, a thread-caching allocator; upstream beats glibc on most 2026 benchmarks, but the crate bundles a 2022-era snapshot.

| | |
|---|---|
| **Beats** | glibc 2.43 malloc on Linux |
| **Claimed result** | README: 'faster than most popular memory allocators like tcmalloc, hoard, ptmalloc3'. mimalloc-bench, rpmalloc develop vs glibc 2.43: leanN 0:07.66 vs 0:09.05, lua 00.70 vs 01.09, sh8benchN 00.24 vs 01.19; rptestN .851 vs .619 (slower). rptest thread sweep, operations per CPU-second: 16 threads 3107756 vs 2353978; 1 thread 19697408 vs 15281704. |
| **Workload** | mimalloc-bench allt suite (real programs and stress tests); rptest with random 16-8000 byte blocks and cross-thread frees, 1 to 16 threads. |
| **Metric** | time |
| **Hardware** | Intel Core i7-13800H, 14 cores / 20 threads, 30 GiB RAM |
| **OS** | Ubuntu 26.04, glibc 2.43 |
| **Toolchain** | Allocators and benchmarks built with -march=native |
| **Measured** | 2026-06 |
| **Adoption** | 161 stars |
| **Last commit** | 2026-10-09 |
| **Platforms** | Tested on x86_64 Windows MSVC, x86_64 macOS and x86_64 Linux (glibc) |
| **Install** | cargo add rpmalloc |
| **Links** | [repository](https://github.com/EmbarkStudios/rpmalloc-rs) · [documentation](https://docs.rs/rpmalloc) · [benchmark](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/BENCHMARKS.md) |

## Verdict

Upstream rpmalloc's 2026 benchmarks show it ahead of glibc on most mimalloc-bench programs (18% on lean, 56% on lua) and 1.3x on its own thread sweep, but behind glibc on the suite's rptest run. Those numbers are for the 2026 develop branch; the rpmalloc crate (0.2.2, 2021) builds the rpmalloc snapshot bundled in rpmalloc-sys 0.2.3, published in 2022, so they do not describe what cargo installs.

**Why tier A:** Public scripts and raw CSV with CPU, OS, glibc version and -march=native stated; first-party, run by rpmalloc's author.

## Use it when

- Game and tool code on Windows, macOS and Linux that wants one allocator everywhere
- Workloads with many small allocations per thread

## Skip it when

- You need the allocator the benchmarks measured: the published crate predates them by four years
- Targets other than the three tested ones (musl and aarch64 are not listed)

## How it is fast

Each thread owns a heap of pages split into size classes, so allocation and same-thread free touch only thread-owned lists. Span and page headers sit at fixed alignments, so free finds them by masking the address instead of a table lookup. Blocks freed by other threads go onto a separate atomic free list per page and are reused by the owner.

## What it costs

- Higher peak memory on several tests (lua: 83076 KB vs glibc's 66628 KB; larsonN: 193568 KB vs 175144 KB).
- No crates.io release since 2021-2022 despite repository activity.

## Problems with the evidence

- Benchmarks are first-party and measure rpmalloc's 2026 develop branch; the newest rpmalloc-sys on crates.io, 0.2.3+b097fd0 (published 2022-04-15), bundles an older upstream snapshot.
- mimalloc-bench runs C programs built with -march=native on one laptop CPU.
- The suite's rptestN entry (glibc faster) and the README's rptest sweep (rpmalloc faster) use different configurations.

## Evidence

<details>
<summary>18 quotes from the sources, checked 2026-10-28</summary>

> We believe rpmalloc is faster than most popular memory allocators like tcmalloc, hoard, ptmalloc3 and others without causing extra allocated memory overhead in the thread caches compared to these allocators.
>
> — [github.com/mjansson/rpmalloc](https://github.com/mjansson/rpmalloc)

> Ubuntu 26.04, glibc 2.43
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/BENCHMARKS.md)

> | rp | rpmalloc | develop |
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/BENCHMARKS.md)

> | sys | system glibc malloc | 2.43 |
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/BENCHMARKS.md)

> leanN rp 0:07.66 224292 18.15 0.39 0 81402
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> leanN sys 0:09.05 242464 21.03 0.52 0 60874
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> lua rp 00.70 83076 5.08 0.48 115 183307
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> lua sys 01.09 66628 5.11 0.61 17964 180175
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> sh8benchN rp 00.24 220084 3.43 0.06 0 56034
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> sh8benchN sys 01.19 347336 19.30 0.65 4 88687
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> rptestN rp .851 136072 3.99 1.47 0 41593
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> rptestN sys .619 69264 2.91 1.94 11 35250
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> larsonN-sized rp 0:2.034 193568 96.36 0.28 2 49311
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> larsonN-sized sys 0:2.326 175144 96.89 0.74 7 53704
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> rp,16,3107756,642
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/rptest-threads.csv)

> sys,16,2353978,642
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/rptest-threads.csv)

> rp,1,19697408,38
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/rptest-threads.csv)

> sys,1,15281704,33
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/rptest-threads.csv)

</details>
