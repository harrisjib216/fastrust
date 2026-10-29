# PGO, LTO and BOLT in the official rustc builds

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md) › [Compile times](README.md)</sub>

**Tier B** (independent measurement) · ✅ Holds up · Ships with Rust · `Rust + C++` · 🟢 active

> The rustc you install with rustup is already built with PGO and LTO (and BOLT on x86_64 Linux), making it 10-30% faster than a plain build.

| | |
|---|---|
| **Beats** | A rustc built with a plain release configuration, without PGO, LTO or BOLT |
| **Claimed result** | PGO-built rustc: 10-16% shorter build times on real-world crates (rust-lang experiment); Windows rustc with PGO: 10-20% better compiler performance (Rust 1.64.0); AArch64 Linux rustc with ThinLTO + PGO: up to 30% faster (Rust 1.86.0); Android's version of the same pipeline: 24.7% faster with codegen-units=1, LTO, PGO and BOLT combined |
| **Workload** | Compiling real-world crates from rustc-perf (webrender, style-servo, serde and others); Android's Rust components for the Android figure |
| **Metric** | build time |
| **Hardware** | Ryzen 1700X (2020 experiment) |
| **OS** | Linux, Windows |
| **Toolchain** | Official rustc dist builds |
| **Measured** | 2025-04 |
| **Platforms** | Official builds for x86_64 Linux, x86_64 Windows and AArch64 Linux, per the release notes quoted |
| **Install** | Built in: use the rustup toolchains |
| **Links** | [documentation](https://github.com/rust-lang/rust/blob/main/RELEASES.md) · [benchmark](https://blog.rust-lang.org/inside-rust/2020/11/11/exploring-pgo-for-the-rust-compiler/) |

## Verdict

The rust-lang project measured 10-16% shorter build times from a PGO-built compiler in 2020 and now ships optimized compilers: Windows with PGO since 1.64 (10-20%), AArch64 Linux with ThinLTO and PGO since 1.86 (up to 30%), matching x86_64 Linux. rustup users already get these gains without doing anything.

**Why tier B:** The rust-lang project measuring its own distribution builds on rustc-perf, plus an independent replication by Google's Android toolchain team.

## Use it when

- Always: use the official rustup toolchains to get these optimizations

## Skip it when

- Distribution-packaged or self-built compilers may not have this pipeline; compare against the rustup build if compile times look slow

## How it is fast

PGO builds rustc and LLVM, runs them on benchmark crates to record which branches and functions are hot, and rebuilds with that profile for better inlining, code layout and branch order. LTO lets the optimizer work across crate and library boundaries, and BOLT reorders the final binary's code for instruction-cache locality.

## What it costs

- None for users; the cost is longer CI builds for the Rust project

## Problems with the evidence

- The 2020 numbers come from one machine
- The 1.64 and 1.86 figures are one-line summaries in the release notes
- The Android numbers measure Android's codebase, which the post says may not reflect other codebases

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> As you can see we get a 10-16% reduction of build times almost across the board for real world test cases.
>
> — [blog.rust-lang.org/inside-rust/2020/11/11/exploring-pgo-for-the-rus…](https://blog.rust-lang.org/inside-rust/2020/11/11/exploring-pgo-for-the-rust-compiler/)

> Windows builds now use profile-guided optimization, providing 10-20% improvements to compiler performance:
>
> — [github.com/rust-lang/rust/blob/main/RELEASES.md](https://github.com/rust-lang/rust/blob/main/RELEASES.md)

> The ARM 64-bit compiler (AArch64) on Linux is now optimized with ThinLTO and PGO, similar to the optimizations we have already performed for the x86-64 compiler on Linux. This should make it up to 30% faster.
>
> — [github.com/rust-lang/rust/blob/main/RELEASES.md](https://github.com/rust-lang/rust/blob/main/RELEASES.md)

> Version 1.64.0 (2022-09-22)
>
> — [github.com/rust-lang/rust/blob/main/RELEASES.md](https://github.com/rust-lang/rust/blob/main/RELEASES.md)

> Version 1.86.0 (2025-04-03)
>
> — [github.com/rust-lang/rust/blob/main/RELEASES.md](https://github.com/rust-lang/rust/blob/main/RELEASES.md)

> Applying BOLT to our toolchain produces a speedup over the baseline compiler of ~24.7% at a size increase of ~10.9%.
>
> — [android-developers.googleblog.com/2023/12/faster-rust-toolchains-fo…](https://android-developers.googleblog.com/2023/12/faster-rust-toolchains-for-android.html)

> To be clear, the optimizations listed below are also present in the versions of rustc that are obtained using rustup
>
> — [android-developers.googleblog.com/2023/12/faster-rust-toolchains-fo…](https://android-developers.googleblog.com/2023/12/faster-rust-toolchains-for-android.html)

</details>
