# BOLT post-link optimization (via cargo-pgo)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md) › [Build settings for faster binaries](README.md)</sub>

**Tier B** (independent measurement) · 🟡 Holds up in part · Open source · `Rust` · `MIT` · 🟢 active

> Profile-driven code layout applied after linking; adds roughly 2-6% on top of PGO in rustc measurements, at a size cost.

| | |
|---|---|
| **Beats** | A PGO-optimized release binary without BOLT |
| **Claimed result** | Android rustc: ~6.1% faster than the PGO-only compiler (~24.7% over baseline) at ~10.9% larger size; rust-lang librustc_driver.so: -1.8% mean cycles across primary benchmarks, with a 34 MiB (+48%) larger library |
| **Workload** | rustc compiling Android's Rust components; rustc-perf primary benchmarks |
| **Metric** | time; binary size |
| **OS** | Linux |
| **Toolchain** | llvm-bolt applied to libLLVM, libstd and librustc_driver after PGO and LTO |
| **Measured** | 2023-12 |
| **Adoption** | 793 stars |
| **Last commit** | 2026-04-25 |
| **Platforms** | Linux ELF binaries; rust-lang disabled BOLT for its AArch64 Linux builds because of upstream BOLT bugs |
| **Install** | cargo install cargo-pgo, plus llvm-bolt and merge-fdata (LLVM 16+ release or the project's Docker image) |
| **Links** | [repository](https://github.com/kobzol/cargo-pgo) · [documentation](https://github.com/Kobzol/cargo-pgo/blob/main/README.md) · [benchmark](https://android-developers.googleblog.com/2023/12/faster-rust-toolchains-for-android.html) |

## Verdict

BOLT adds a further gain on top of PGO, but the measured gains are small and vary: about 6.1% for Android's rustc and a 1.8% mean cycle reduction for librustc_driver.so in rust-lang CI. It enlarges binaries, needs relocations kept in the binary, and cargo-pgo marks its BOLT support experimental.

**Why tier B:** Measured by Google's Android toolchain team and the rust-lang project on rustc, independent of cargo-pgo.

## Use it when

- Large Linux binaries already built with PGO
- Programs limited by instruction-cache misses, such as compilers and databases

## Skip it when

- Binary size limits
- Non-Linux targets
- Release pipelines that strip symbols: cargo-pgo warns against stripping when using BOLT

## How it is fast

BOLT rewrites the linked binary using a profile of which functions and blocks run hot, placing hot code together and moving cold code away. That cuts instruction-cache and page misses in large programs whose hot paths are spread across many functions, which the linker cannot see without a profile.

## What it costs

- Binary size grew 10.9% (Android) and 48% for librustc_driver.so (rust-lang)
- Extra build step with experimental tooling

## Problems with the evidence

- Both measurements are on rustc
- The rust-lang -1.8% is a cycle count, not wall time

## Evidence

<details>
<summary>5 quotes from the sources, checked 2026-10-28</summary>

> Applying BOLT to our toolchain produces a speedup over the baseline compiler of ~24.7% at a size increase of ~10.9%. This is a speedup of ~6.1% over the PGOed compiler without BOLT.
>
> — [android-developers.googleblog.com/2023/12/faster-rust-toolchains-fo…](https://android-developers.googleblog.com/2023/12/faster-rust-toolchains-for-android.html)

> -1.8% mean improvement in cycle counts across many primary benchmarks.
>
> — [github.com/rust-lang/rust/pull/116352](https://github.com/rust-lang/rust/pull/116352)

> 34 MiB (+48%) artifact size regression of
>
> — [github.com/rust-lang/rust/pull/116352](https://github.com/rust-lang/rust/pull/116352)

> BOLT and Docker support is currently *experimental*.
>
> — [github.com/Kobzol/cargo-pgo/blob/main/README.md](https://github.com/Kobzol/cargo-pgo/blob/main/README.md)

> For the time being, disable bolt on aarch64 due to upstream bolt bugs.
>
> — [github.com/rust-lang/rust/pull/133807](https://github.com/rust-lang/rust/pull/133807)

</details>
