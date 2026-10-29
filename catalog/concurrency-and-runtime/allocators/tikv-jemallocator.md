# tikv-jemallocator (jemalloc global allocator)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md) › [Memory allocators](README.md)</sub>

**Tier B** (independent measurement) · 🟡 Holds up in part · Open source · `Rust + C` · `MIT/Apache-2.0` · 🟢 active

> Maintained jemalloc bindings that rustc itself uses; 13-14% faster rustc builds on macOS, mixed results against modern glibc.

| | |
|---|---|
| **Beats** | System allocator: macOS libmalloc (rustc builds), glibc 2.43 malloc, musl malloc |
| **Claimed result** | rustc with jemalloc on macOS: cargo build 66.658 s vs 75.251 s ('1.13 ± 0.01 times faster'), toml-rs build 5.968 s vs 6.777 s ('1.14 ± 0.01 times faster'). mimalloc-bench, jemalloc 5.3.0 vs glibc 2.43: leanN 0:07.78 vs 0:09.05 and espresso 0:02.97 vs 0:03.23 (faster); redis 0:2.619 vs 0:2.240 and xmalloc-testN 2.573 vs 0.739 (slower). |
| **Workload** | Building cargo and toml-rs from scratch with a nightly rustc whose allocator was or was not jemalloc (hyperfine, 2 runs each); mimalloc-bench allt suite on glibc. |
| **Metric** | time |
| **Hardware** | Intel Mac, model not stated (rustc); Intel Core i7-13800H (mimalloc-bench) |
| **OS** | macOS on x86_64-apple-darwin (rustc); Ubuntu 26.04 with glibc 2.43 (mimalloc-bench) |
| **Toolchain** | nightly-2021-03-08 vs the next nightly with PR #82642 (rustc); -march=native (mimalloc-bench) |
| **Measured** | 2021-03-09 |
| **Adoption** | 555 stars |
| **Last commit** | 2026-10-10 |
| **Platforms** | Linux (x86_64, aarch64, powerpc64le tested), macOS (aarch64 tested), musl; not Windows MSVC |
| **Install** | cargo add tikv-jemallocator |
| **Links** | [repository](https://github.com/tikv/jemallocator) · [documentation](https://docs.rs/tikv-jemallocator) · [benchmark](https://github.com/rust-lang/rust/pull/82642) |

## Verdict

On macOS, rust-lang measured cargo and toml-rs builds 13-14% faster once rustc's jemalloc was actually wired in, and on musl it avoids the slow default allocator. On glibc the picture is mixed: jemalloc 5.3.0 was faster on lean and espresso but slower on redis and 3.5x slower on the cross-thread xmalloc test, and ripgrep's author found no meaningful difference for its workload.

**Why tier B:** The rust-lang project measuring rustc with hyperfine, plus an independent mimalloc-bench run; the rustc numbers come from two runs per configuration.

## Use it when

- Static musl builds (ripgrep switches to it on musl for this reason)
- macOS programs whose allocation is heavy, as with rustc
- Long-running services that need jemalloc's heap profiling and statistics

## Skip it when

- glibc Linux programs that are not allocation-bound: ripgrep saw no meaningful difference
- Producer-consumer patterns that free on another thread, where jemalloc 5.3.0 trailed glibc on xmalloc-test
- Windows MSVC targets

## How it is fast

Per-thread caches serve small allocations without locks, and memory is spread over several arenas to cut contention. The Rust wrapper passes the allocation size to sdallocx on free, which skips a metadata lookup that plain free() needs.

## What it costs

- Longer build: jemalloc is compiled from C source; ripgrep's author notes it increases compile times.
- Different memory profile: in mimalloc-bench redis peaked at 8156 KB vs glibc's 6324 KB.

## Problems with the evidence

- The macOS numbers are two runs each on one unnamed Intel Mac, measuring rustc rather than a typical Rust program.
- In 2021 rustc linked jemalloc through the jemalloc-sys crate; it switched to tikv-jemalloc-sys later.
- mimalloc-bench runs C programs built with -march=native, not Rust.

## Evidence

<details>
<summary>20 quotes from the sources, checked 2026-10-28</summary>

> I did some basic benchmarks, and it looks like this had a fairly significant improvement!
>
> — [github.com/rust-lang/rust/pull/82642](https://github.com/rust-lang/rust/pull/82642)

> Time (mean ± σ): 66.658 s ± 0.394 s [User: 299.565 s, System: 27.624 s]
>
> — [github.com/rust-lang/rust/pull/82642](https://github.com/rust-lang/rust/pull/82642)

> Time (mean ± σ): 75.251 s ± 0.283 s [User: 337.470 s, System: 28.521 s]
>
> — [github.com/rust-lang/rust/pull/82642](https://github.com/rust-lang/rust/pull/82642)

> 1.13 ± 0.01 times faster than
>
> — [github.com/rust-lang/rust/pull/82642](https://github.com/rust-lang/rust/pull/82642)

> Time (mean ± σ): 5.968 s ± 0.010 s [User: 7.561 s, System: 0.493 s]
>
> — [github.com/rust-lang/rust/pull/82642](https://github.com/rust-lang/rust/pull/82642)

> Time (mean ± σ): 6.777 s ± 0.049 s [User: 8.598 s, System: 0.509 s]
>
> — [github.com/rust-lang/rust/pull/82642](https://github.com/rust-lang/rust/pull/82642)

> 1.14 ± 0.01 times faster than
>
> — [github.com/rust-lang/rust/pull/82642](https://github.com/rust-lang/rust/pull/82642)

> nightly-2021-03-08-x86_64-apple-darwin
>
> — [github.com/rust-lang/rust/pull/82642](https://github.com/rust-lang/rust/pull/82642)

> Ubuntu 26.04, glibc 2.43
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/BENCHMARKS.md)

> | je | jemalloc | 5.3.0 |
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/BENCHMARKS.md)

> leanN je 0:07.78 214556 18.36 0.39 0 63225
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> leanN sys 0:09.05 242464 21.03 0.52 0 60874
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> espresso je 0:02.97 5856 2.96 0.01 0 982
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> espresso sys 0:03.23 5672 3.22 0.00 42 1003
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> redis je 0:2.619 8156 0.11 0.02 0 1774
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> redis sys 0:2.240 6324 0.09 0.02 0 1610
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> xmalloc-testN je 2.573 104768 93.98 0.64 73 56706
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> xmalloc-testN sys 0.739 59672 56.99 35.66 5 503986
>
> — [github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1…](https://github.com/mjansson/rpmalloc/blob/a6033ae33ffc816db07eebce5b5efc9e1f55ba2f/benchmark/results/mimalloc-bench-allt.csv)

> difference (for ripgrep's purposes) between glibc's allocator and jemalloc.
>
> — [github.com/BurntSushi/ripgrep/blob/master/crates/core/main.rs](https://github.com/BurntSushi/ripgrep/blob/master/crates/core/main.rs)

> allocator, which appears to be substantially worse.
>
> — [github.com/BurntSushi/ripgrep/blob/master/crates/core/main.rs](https://github.com/BurntSushi/ripgrep/blob/master/crates/core/main.rs)

</details>
