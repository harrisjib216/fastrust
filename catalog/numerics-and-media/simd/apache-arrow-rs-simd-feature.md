# arrow-rs `simd` feature (hand-written packed_simd arithmetic kernels)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Numerics and media](../README.md) › [SIMD](README.md)</sub>

**Tier A** (reproducible public benchmark) · ❌ Doesn't hold up · Open source · `Rust` · `Apache-2.0` · 🔴 abandoned

> Arrow's hand-written packed_simd kernels lost to auto-vectorized scalar loops (1.5-2.1x slower) and were removed; the default won.

| | |
|---|---|
| **Beats** | arrow's scalar arithmetic kernels left to LLVM's auto-vectorizer |
| **Claimed result** | Criterion medians, 64k-element arrays, with simd feature vs without: add 27.895 us vs 13.485 us, subtract 27.194 us vs 17.829 us, multiply 27.872 us vs 16.638 us (auto-vectorized 2.07x, 1.53x and 1.68x faster) (derived from page data); checked divide 46.385 us vs 104.55 us (explicit SIMD 2.25x faster) (derived from page data) |
| **Workload** | arrow arithmetic_kernels criterion benchmarks on primitive arrays of 64k elements: add, subtract, multiply, divide, modulo, with and without nulls. |
| **Metric** | time |
| **Hardware** | AMD Ryzen 3700U laptop |
| **Toolchain** | Nightly rustc of January 2022 (packed_simd required nightly); RUSTFLAGS="-C target-cpu=skylake" |
| **Measured** | 2022-01-15 |
| **Adoption** | 3.6k stars |
| **Last commit** | 2026-10-28 |
| **Platforms** | Was nightly-only (packed_simd); removed in December 2023 |
| **Install** | No longer available: the simd feature was removed from arrow |
| **Links** | [repository](https://github.com/apache/arrow-rs) · [documentation](https://github.com/apache/arrow-rs/pull/5184) · [benchmark](https://github.com/apache/arrow-rs/issues/1182) |

## Verdict

Arrow's own measurements showed the scalar, auto-vectorized add, subtract and multiply kernels running 1.5x to 2.1x faster than its explicit SIMD versions, so the project deleted them in 2022 and removed the whole simd feature in December 2023. Explicit SIMD kept a lead only for checked division, where the scalar loop has a branch per element.

**Why tier A:** The arrow project measured its own change with public criterion benchmarks, stating the CPU, array size and target-cpu flag.

## Use it when

- Treat this as evidence for the default: write branch-free scalar loops over slices and let LLVM vectorize them before reaching for explicit SIMD.

## Skip it when

- Do not assume hand-written SIMD beats the auto-vectorizer for element-wise arithmetic; measure first.

## How it is fast

It was not faster for simple arithmetic. The explicit kernels processed a fixed number of lanes per iteration, while LLVM unrolled the plain loops several times and checked the loop condition less often. For checked division the scalar loop branches on zero divisors per element, which blocks auto-vectorization, so the explicit SIMD kernel kept its lead there.

## What it costs

- Relying on the auto-vectorizer means performance can silently drop when a code change adds a branch or blocks inlining.

## Problems with the evidence

- The numbers come from one laptop with target-cpu=skylake; with default x86_64 flags the auto-vectorizer can only use SSE2, which narrows what either approach can do.
- The explicit kernels used packed_simd, the nightly-only predecessor of std::simd; a std::simd rewrite was never benchmarked against the auto-vectorized code.
- Arrow's CONTRIBUTING guide lists the conditions for auto-vectorization to work: no per-row conditionals, limited inlining, no horizontal reductions, and suitable target-cpu.

## Evidence

<details>
<summary>16 quotes from the sources, checked 2026-10-28</summary>

> Benchmarks with array size of 64k, run on an AMD Ryzen 3700U laptop.
>
> — [github.com/apache/arrow-rs/issues/1182](https://github.com/apache/arrow-rs/issues/1182)

> RUSTFLAGS="-C target-cpu=skylake"
>
> — [github.com/apache/arrow-rs/issues/1182](https://github.com/apache/arrow-rs/issues/1182)

> With simd feature:
>
> — [github.com/apache/arrow-rs/issues/1182](https://github.com/apache/arrow-rs/issues/1182)

> add time: [27.039 us 27.895 us 28.542 us]
>
> — [github.com/apache/arrow-rs/issues/1182](https://github.com/apache/arrow-rs/issues/1182)

> subtract time: [26.408 us 27.194 us 28.040 us]
>
> — [github.com/apache/arrow-rs/issues/1182](https://github.com/apache/arrow-rs/issues/1182)

> multiply time: [27.303 us 27.872 us 28.429 us]
>
> — [github.com/apache/arrow-rs/issues/1182](https://github.com/apache/arrow-rs/issues/1182)

> divide time: [45.050 us 46.385 us 47.590 us]
>
> — [github.com/apache/arrow-rs/issues/1182](https://github.com/apache/arrow-rs/issues/1182)

> Without simd feature
>
> — [github.com/apache/arrow-rs/issues/1182](https://github.com/apache/arrow-rs/issues/1182)

> add time: [12.703 us 13.485 us 14.165 us]
>
> — [github.com/apache/arrow-rs/issues/1182](https://github.com/apache/arrow-rs/issues/1182)

> subtract time: [17.586 us 17.829 us 18.018 us]
>
> — [github.com/apache/arrow-rs/issues/1182](https://github.com/apache/arrow-rs/issues/1182)

> multiply time: [16.261 us 16.638 us 17.035 us]
>
> — [github.com/apache/arrow-rs/issues/1182](https://github.com/apache/arrow-rs/issues/1182)

> divide time: [97.142 us 104.55 us 111.13 us]
>
> — [github.com/apache/arrow-rs/issues/1182](https://github.com/apache/arrow-rs/issues/1182)

> Removes explicit simd arithmetic kernels, except for division/modulo, as compiler autovectorization actually generates better code.
>
> — [github.com/apache/arrow-rs/pull/1221](https://github.com/apache/arrow-rs/pull/1221)

> removed the last major use of SIMD, so we can now remove this feature
>
> — [github.com/apache/arrow-rs/pull/5184](https://github.com/apache/arrow-rs/pull/5184)

> packed_simd = { version = "0.3", optional = true, package = "packed_simd_2" }
>
> — [github.com/apache/arrow-rs/blob/2d6352b3b35a17cef2618f6475b7230ce01…](https://github.com/apache/arrow-rs/blob/2d6352b3b35a17cef2618f6475b7230ce0146d55/arrow/Cargo.toml)

> This crate used to contain several kernels with hand-written SIMD instructions, which were removed after discovering the auto-vectorized code was faster.
>
> — [github.com/apache/arrow-rs/blob/70d8190350ffb2fd3a34c92d8844d505eaf…](https://github.com/apache/arrow-rs/blob/70d8190350ffb2fd3a34c92d8844d505eaf0af75/arrow/CONTRIBUTING.md)

</details>
