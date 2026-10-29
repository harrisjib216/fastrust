# std::simd (portable SIMD, nightly only)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Numerics and media](../README.md) › [SIMD](README.md)</sub>

**Tier C** (first-party numbers) · 🟡 Holds up in part · Ships with Rust · `Rust` · 🟢 active

> Portable SIMD vector types in std; can beat auto-vectorized loops, but still nightly-only and with little published Rust benchmark data.

| | |
|---|---|
| **Beats** | Scalar Rust loops left to LLVM's auto-vectorizer (in the cited case, RangeSetBlaze::from_iter) |
| **Claimed result** | range-set-blaze from_slice (core::simd) vs from_iter: "On a representative benchmark, the speed up was 7×." |
| **Workload** | Building a RangeSetBlaze from a slice of "clumpy" integers (long runs of consecutive values); the SIMD path detects runs of consecutive integers in vector registers. |
| **Metric** | time |
| **Toolchain** | Nightly rustc with #![feature(portable_simd)]; the crate recommends -C target-cpu=native |
| **Platforms** | All targets (lowers to SSE/AVX, NEON, simd128 or scalar); nightly toolchain only |
| **Install** | Nightly only: #![feature(portable_simd)] (tracking issue rust-lang/rust#86656) |
| **Links** | [documentation](https://doc.rust-lang.org/std/simd/index.html) · [benchmark](https://github.com/CarlKCarlK/range-set-blaze/blob/main/src/set.rs) |

## Verdict

std::simd can beat scalar code, but it is still nightly-only in October 2026, and the published Rust numbers are thin: the clearest is a single crate reporting a 7x faster constructor on clumpy integers, which also changes the algorithm and asks for target-cpu=native. Arrow found hand-written SIMD slower than the auto-vectorizer for plain element-wise arithmetic.

**Why tier C:** The only numeric claim found is first-party, from one crate, with no hardware or benchmark detail next to it.

## Use it when

- You already build with nightly and have a loop the auto-vectorizer cannot handle, such as a float reduction, a byte search or a run detection.

## Skip it when

- You need stable Rust: use wide, pulp or std::arch intrinsics instead.
- Your loop is simple element-wise arithmetic, which LLVM already vectorizes; arrow measured its explicit SIMD kernels slower there.
- You ship generic x86_64 binaries: without target-cpu or multiversioning, std::simd is limited to SSE2.

## How it is fast

Simd<T, N> lowers to the target's vector instructions through LLVM's portable vector IR, so one source compiles to SSE, AVX, NEON or wasm simd128. It lets you express what the auto-vectorizer will not do on its own: reductions over floats, lane shuffles, comparisons turned into masks, and early exits across a whole vector.

## What it costs

- Nightly toolchain required, with API churn.
- Benefits beyond SSE2 on x86_64 require target-cpu flags or function multiversioning.

## Problems with the evidence

- The 7x figure is the crate author's own, with no hardware, input size or toolchain given, and the SIMD constructor uses a different algorithm (scan for runs, then build), so it is not a pure scalar-vs-SIMD comparison.
- portable_simd tracking issue #86656 is still open; the API can change between nightlies.
- std::simd chooses instructions at compile time; runtime feature detection needs a crate such as multiversion.

## Evidence

<details>
<summary>6 quotes from the sources, checked 2026-10-28</summary>

> On a representative benchmark, the speed up was 7×.
>
> — [github.com/CarlKCarlK/range-set-blaze/blob/2d96087a6d2d4cedb07031a7…](https://github.com/CarlKCarlK/range-set-blaze/blob/2d96087a6d2d4cedb07031a75b33cc5eef5b24aa/src/set.rs)

> It is nightly only and enabled by the `from_slice` feature.
>
> — [github.com/CarlKCarlK/range-set-blaze/blob/2d96087a6d2d4cedb07031a7…](https://github.com/CarlKCarlK/range-set-blaze/blob/2d96087a6d2d4cedb07031a75b33cc5eef5b24aa/src/set.rs)

> To enable SIMD optimizations, compile with the Rust compiler
>
> — [github.com/CarlKCarlK/range-set-blaze/blob/2d96087a6d2d4cedb07031a7…](https://github.com/CarlKCarlK/range-set-blaze/blob/2d96087a6d2d4cedb07031a75b33cc5eef5b24aa/src/set.rs)

> This is a nightly-only experimental API.
>
> — [doc.rust-lang.org/std/simd/index.html](https://doc.rust-lang.org/std/simd/index.html)

> Tracking Issue for RFC 2948: Portable SIMD
>
> — [github.com/rust-lang/rust/issues/86656](https://github.com/rust-lang/rust/issues/86656)

> This crate used to contain several kernels with hand-written SIMD instructions, which were removed after discovering the auto-vectorized code was faster.
>
> — [github.com/apache/arrow-rs/blob/70d8190350ffb2fd3a34c92d8844d505eaf…](https://github.com/apache/arrow-rs/blob/70d8190350ffb2fd3a34c92d8844d505eaf0af75/arrow/CONTRIBUTING.md)

</details>
