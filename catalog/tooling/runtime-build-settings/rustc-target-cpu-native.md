# -C target-cpu=native

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md) › [Build settings for faster binaries](README.md)</sub>

**Tier B** (independent measurement) · 🟡 Holds up in part · Ships with Rust · `Rust` · 🟢 active

> Compiling for the build machine's exact CPU helps vectorizable loops; for general code measured gains were nil or negative.

| | |
|---|---|
| **Beats** | cargo build --release for the target's default generic CPU |
| **Claimed result** | Rust Performance Book: can improve runtime speed, especially with vectorization opportunities (no numbers). rustc built for x86-64-v3: instruction counts down up to -2.5% (inflate-check) and -6.6% (keccak-debug), but 'a pretty significant wall time loss' overall; x86-64-v2 'a slight win (or perhaps lost in the noise)' |
| **Workload** | The Rust compiler built with -Ctarget-cpu=x86-64-v2 and x86-64-v3, timed on the rustc-perf suite |
| **Metric** | time |
| **Hardware** | rustc-perf benchmark machine (not stated) |
| **OS** | Linux |
| **Toolchain** | rustc with a backported LLVM patch adding x86-64-v2/v3/v4 CPU levels (November 2020) |
| **Measured** | 2020-11 |
| **Platforms** | All targets; 'native' means the build machine's CPU |
| **Install** | RUSTFLAGS="-C target-cpu=native" cargo build --release |
| **Links** | [documentation](https://nnethercote.github.io/perf-book/build-configuration.html) · [benchmark](https://github.com/rust-lang/rust/pull/79043) |

## Verdict

It helps code with loops LLVM can vectorize, but it is not a general speedup. When the rust-lang project built rustc for x86-64-v3, the reviewer called the result a significant wall-time loss despite instruction-count gains of up to 2.5-6.6% on a few benchmarks, and native binaries can fail on older CPUs.

**Why tier B:** A rust-lang project measurement on rustc-perf, though from 2020 (LLVM 11 era) and using x86-64-v3 rather than native; the Rust Performance Book gives no numbers.

## Use it when

- Binaries built and run on the same machine or a uniform fleet
- Numeric or data-parallel loops that LLVM can auto-vectorize

## Skip it when

- Distributing binaries: they can fail with illegal-instruction errors on older CPUs
- Branchy, pointer-heavy code such as compilers and parsers, where the measured effect was nil or negative
- Cross-compiling, where 'native' describes the wrong machine

## How it is fast

target-cpu=native lets LLVM use every instruction-set extension on the build machine, such as AVX2 or AVX-512, and tune for that CPU. Loops that auto-vectorize can process more data per instruction; branchy code with little data parallelism changes little.

## What it costs

- Non-portable binaries
- Output depends on the build machine, so builds differ across machines

## Problems with the evidence

- The only measured numbers found are from 2020 and target x86-64-v3, not native
- The test program was rustc; numeric code may benefit more

## Evidence

<details>
<summary>4 quotes from the sources, checked 2026-10-28</summary>

> This can improve runtime speed, especially if the compiler finds vectorization opportunities in your code.
>
> — [nnethercote.github.io/perf-book/build-configuration.html](https://nnethercote.github.io/perf-book/build-configuration.html)

> This PR instructs rustbuild to compile the compiler with the -Ctarget-cpu=x86-64-v2 option enabled in hope of getting some optimization gains from autovectorization.
>
> — [github.com/rust-lang/rust/pull/79043](https://github.com/rust-lang/rust/pull/79043)

> V3 looks like a pretty significant wall time loss, V2 looks like a slight win (or perhaps lost in the noise). My guess is that this is not worth it at this time.
>
> — [github.com/rust-lang/rust/pull/79043](https://github.com/rust-lang/rust/pull/79043)

> Yeah there are some single digit improvements in instruction counts (up to -2.5% in inflate-check full for v3, up to -6.6% in keccak-debug full)
>
> — [github.com/rust-lang/rust/pull/79043](https://github.com/rust-lang/rust/pull/79043)

</details>
