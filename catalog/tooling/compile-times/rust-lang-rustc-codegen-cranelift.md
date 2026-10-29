# rustc_codegen_cranelift (Cranelift codegen backend)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md) › [Compile times](README.md)</sub>

**Tier B** (independent measurement) · 🟡 Holds up in part · Ships with Rust · `Rust` · `Apache-2.0` · 🟢 active

> Nightly-only codegen backend that compiles debug builds 5-20% faster than LLVM by generating less optimized code.

| | |
|---|---|
| **Beats** | cargo build (dev profile) with the default LLVM backend |
| **Claimed result** | Zed incremental build: 5% faster with debuginfo off, 12% faster with line-tables-only (February 2026, 16-core machine); single-crate builds in the rustc benchmark suite: negligible to ~30%, average 10-15%. LWN: full debug build of Cranelift 29.6 s vs 37.5 s with LLVM (20% less wall-clock), 125 vs 211 CPU-seconds (40% less) |
| **Workload** | Start-to-finish incremental compilation of Zed; single-crate compiles in the rustc benchmark suite; a full debug build of the Cranelift crate with its dependencies |
| **Metric** | build time |
| **Hardware** | 16-core machine (2026 goal); unspecified machine (LWN) |
| **Toolchain** | nightly rustc with the rustc-codegen-cranelift-preview component; -Cdebuginfo=none and -Cdebuginfo=line-tables-only |
| **Measured** | 2026-02 |
| **Adoption** | 2.2k stars |
| **Last commit** | 2026-10-28 |
| **Platforms** | Nightly builds on Linux, macOS and x86_64 Windows |
| **Install** | rustup component add rustc-codegen-cranelift-preview --toolchain nightly |
| **Links** | [repository](https://github.com/rust-lang/rustc_codegen_cranelift) · [documentation](https://github.com/rust-lang/rustc_codegen_cranelift/blob/main/Readme.md) · [benchmark](https://goals.rust-lang.org/2026/improve-cg_clif-performance.html) |

## Verdict

Cranelift does compile debug builds faster than LLVM, but the measured gains are modest: 5-12% on Zed incremental builds and 10-15% per crate on average in early 2026, and 20% wall-clock on a 2024 third-party full build. It is still nightly-only and produces slower code, so it fits dev builds only.

**Why tier B:** LWN's 2024 measurement is independent of the backend's author; the 2026 numbers come from the author in a rust-lang project goal, with only the core count stated.

## Use it when

- Dev builds on nightly where code generation is a large share of rebuild time
- Projects that avoid the features its README lists as not yet supported

## Skip it when

- Release builds: the generated code is slower
- Stable-only toolchains
- Code using std::arch SIMD intrinsics or relying on unwinding on Windows or macOS

## How it is fast

Cranelift is a code generator built to compile quickly rather than to produce the fastest code; a 2020 paper cited by LWN found it an order of magnitude faster than LLVM with code about twice as slow on some benchmarks. rustc hands it the same program it would hand LLVM, so less time goes into code generation. Parsing, type checking, linking and debuginfo are unchanged, which limits the end-to-end gain.

## What it costs

- Nightly only
- Slower generated code
- Partial SIMD support; panic=abort by default

## Problems with the evidence

- The 2026 numbers are the backend author's, on one 16-core machine
- LWN measured one crate on an unspecified machine
- The front end, linker and debuginfo are unchanged, which caps the end-to-end gain

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> In local benchmarks on a 16-core machine, as of February 2026, measuring start-to-finish incremental compilation time of Zed:
>
> — [goals.rust-lang.org/2026/improve-cg_clif-performance.html](https://goals.rust-lang.org/2026/improve-cg_clif-performance.html)

> cranelift is 5% faster than LLVM
>
> — [goals.rust-lang.org/2026/improve-cg_clif-performance.html](https://goals.rust-lang.org/2026/improve-cg_clif-performance.html)

> cranelift is 12% faster than LLVM
>
> — [goals.rust-lang.org/2026/improve-cg_clif-performance.html](https://goals.rust-lang.org/2026/improve-cg_clif-performance.html)

> the performance benefit of the cranelift backend varies between negligible and ~30% with the average speedup around 10-15% for a single crate compilation in
>
> — [goals.rust-lang.org/2026/improve-cg_clif-performance.html](https://goals.rust-lang.org/2026/improve-cg_clif-performance.html)

> A full debug build of Cranelift itself using the Cranelift backend took 29.6 seconds on my computer, compared to 37.5 with LLVM (a reduction in wall-clock time of 20%).
>
> — [lwn.net/Articles/964735/](https://lwn.net/Articles/964735/)

> Compiling with Cranelift took 125 CPU-seconds, whereas LLVM took 211 CPU-seconds, a difference of 40%.
>
> — [lwn.net/Articles/964735/](https://lwn.net/Articles/964735/)

> The Cranelift codegen backend is distributed in nightly builds on Linux, macOS and x86_64 Windows.
>
> — [github.com/rust-lang/rustc_codegen_cranelift/blob/main/Readme.md](https://github.com/rust-lang/rustc_codegen_cranelift/blob/main/Readme.md)

</details>
