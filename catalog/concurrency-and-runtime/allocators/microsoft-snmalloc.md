# snmalloc-rs (snmalloc global allocator)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md) › [Memory allocators](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Open source · `Rust + C++` · `MIT` · 🟢 active

> Rust bindings for Microsoft's message-passing allocator; 7.2x faster than glibc on cross-thread frees and 4-18% faster on real programs.

| | |
|---|---|
| **Beats** | glibc 2.43 malloc on Linux |
| **Claimed result** | mimalloc-bench, snmalloc 0.7.4 vs glibc 2.43: xmalloc-testN 0.102 vs 0.739, sh8benchN 00.25 vs 01.19, leanN 0:07.66 vs 0:09.05, espresso 0:02.85 vs 0:03.23, redis 0:2.160 vs 0:2.240; jemalloc took 2.573 on xmalloc-testN. |
| **Workload** | mimalloc-bench allt suite: real programs (espresso, lean, redis) and multithreaded stress tests, including xmalloc-test, where one set of threads allocates and another frees. |
| **Metric** | time |
| **Hardware** | Intel Core i7-13800H, 14 cores / 20 threads, 30 GiB RAM |
| **OS** | Ubuntu 26.04, glibc 2.43 |
| **Toolchain** | Allocators and benchmarks built with -march=native |
| **Measured** | 2026-06 |
| **Adoption** | 2.0k stars |
| **Last commit** | 2026-10-19 |
| **Platforms** | Linux and BSDs (ELF platforms named in the README) and other targets snmalloc supports; default build needs CMake and a C++ compiler |
| **Install** | cargo add snmalloc-rs |
| **Links** | [repository](https://github.com/microsoft/snmalloc) · [documentation](https://docs.rs/snmalloc-rs) · [benchmark](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/BENCHMARKS.md) |

## Verdict

snmalloc's claim that frees on another thread do not slow it down holds: in an independent 2026 mimalloc-bench run it was the fastest allocator on the cross-thread xmalloc test, 7.2x faster than glibc and 25x faster than jemalloc. On real programs it beat glibc by 4% to 18%, in line with mimalloc.

**Why tier A:** A third-party mimalloc-bench run with public scripts and raw CSV, CPU, OS, glibc version and -march=native stated.

## Use it when

- Producer-consumer pipelines where one thread allocates and another frees (channels, work queues)
- Many-core servers with heavy cross-thread object hand-off

## Skip it when

- Memory-tight deployments: on xmalloc-testN it peaked at 158908 KB vs glibc's 59672 KB
- Builds that cannot depend on CMake and a C++ toolchain

## How it is fast

Memory freed by its allocating thread needs no synchronization. Memory freed by another thread is batched into messages and returned to the owning allocator's queue, so thousands of remote frees cost one atomic operation. The malloc fast path is two branches on Linux when compiled with Clang.

## What it costs

- Higher peak memory on some stress tests (larsonN: 244528 KB vs glibc's 175144 KB).
- C++ build dependency.

## Problems with the evidence

- mimalloc-bench runs C and C++ programs, not Rust, built with -march=native on one laptop CPU.
- The benchmark used snmalloc 0.7.4; the current snmalloc-rs crate is 0.7.6.
- The crate moved from SchrodingerZhu/snmalloc-rs (now archived) into the snmalloc repository.

## Evidence

<details>
<summary>18 quotes from the sources, checked 2026-10-28</summary>

> Allocations on one thread are freed by a different thread
>
> — [github.com/microsoft/snmalloc](https://github.com/microsoft/snmalloc)

> Both of these can cause massive reductions in performance of other allocators, but do not for snmalloc.
>
> — [github.com/microsoft/snmalloc](https://github.com/microsoft/snmalloc)

> 13th Gen Intel Core i7-13800H, 14 cores / 20 threads
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/BENCHMARKS.md)

> Ubuntu 26.04, glibc 2.43
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/BENCHMARKS.md)

> | sn, sn-sec | snmalloc | 0.7.4 |
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/BENCHMARKS.md)

> xmalloc-testN sn 0.102 158908 94.78 1.04 22 40347
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> xmalloc-testN sys 0.739 59672 56.99 35.66 5 503986
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> xmalloc-testN je 2.573 104768 93.98 0.64 73 56706
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> sh8benchN sn 00.25 140692 3.96 0.05 0 35664
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> sh8benchN sys 01.19 347336 19.30 0.65 4 88687
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> leanN sn 0:07.66 243092 17.90 0.36 0 59605
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> leanN sys 0:09.05 242464 21.03 0.52 0 60874
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> espresso sn 0:02.85 5792 2.84 0.01 0 1064
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> espresso sys 0:03.23 5672 3.22 0.00 42 1003
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> redis sn 0:2.160 6784 0.10 0.01 0 1646
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> redis sys 0:2.240 6324 0.09 0.02 0 1610
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> larsonN-sized sn 0:2.279 244528 97.17 0.31 4 61534
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> larsonN-sized sys 0:2.326 175144 96.89 0.74 7 53704
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

</details>
