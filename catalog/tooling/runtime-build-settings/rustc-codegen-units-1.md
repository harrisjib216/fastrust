# codegen-units = 1 in the release profile

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md) › [Build settings for faster binaries](README.md)</sub>

**Tier B** (independent measurement) · 🟡 Holds up in part · Ships with Rust · `Rust` · 🟢 active

> Compiling each crate as one LLVM unit gains about 1.5-2% runtime and shrinks binaries, but roughly doubles release build time.

| | |
|---|---|
| **Beats** | cargo build --release with the default codegen-units = 16 and thin-local LTO |
| **Claimed result** | rustc built with one codegen unit: wall-time -1.57% mean, cycles -1.91%, librustc_driver size -31.6%; Android rustc: ~1.8% faster and ~5.5% smaller, with ~2x longer toolchain builds |
| **Workload** | The Rust compiler as the optimized program, timed compiling the rustc-perf suite (rust-lang) and Android's Rust components (Android) |
| **Metric** | time; binary size |
| **OS** | Linux |
| **Toolchain** | rustc x86_64 Linux dist build (already using LTO, PGO and BOLT) with codegen-units=1 |
| **Measured** | 2023-10 |
| **Platforms** | All targets |
| **Install** | [profile.release] codegen-units = 1 |
| **Links** | [documentation](https://doc.rust-lang.org/cargo/reference/profiles.html) · [benchmark](https://github.com/rust-lang/rust/pull/115554) |

## Verdict

Two large-program measurements show a small runtime gain: 1.57% mean wall-time on rustc's benchmark suite and about 1.8% for Android's toolchain. The bigger effect is binary size (librustc_driver 31.6% smaller), and the cost is a roughly 2x longer build.

**Why tier B:** Measured by the rust-lang project on rustc-perf and by Google's Android toolchain team, both using the Rust compiler as the program under test.

## Use it when

- Final release builds where binary size matters or a 1-2% gain is worth a longer build
- CI release jobs rather than local iteration

## Skip it when

- Iterative release builds: build time roughly doubled in Android's measurement
- Crates where LLVM time is small and inlining across units rarely matters

## How it is fast

Cargo splits each crate into 16 codegen units in release builds so LLVM can optimize them in parallel, but each unit is optimized without seeing the others, which blocks some inlining. With one unit LLVM sees the whole crate at once and can inline and drop dead code more freely, at the cost of serial LLVM work.

## What it costs

- Longer, less parallel release builds
- Smaller binaries (a side benefit)

## Problems with the evidence

- Both measurements use rustc, which already used LTO and PGO; gains on other programs may differ
- The rust-lang numbers are means across many benchmarks; individual workloads vary

## Evidence

<details>
<summary>5 quotes from the sources, checked 2026-10-28</summary>

> wall-time: -1.57% (ditto)
>
> — [github.com/rust-lang/rust/pull/115554](https://github.com/rust-lang/rust/pull/115554)

> cycles: -1.91% (ditto)
>
> — [github.com/rust-lang/rust/pull/115554](https://github.com/rust-lang/rust/pull/115554)

> librustc_driver size: -31.6%
>
> — [github.com/rust-lang/rust/pull/115554](https://github.com/rust-lang/rust/pull/115554)

> To expose as many opportunities for optimization as possible and ensure reproducible builds we add the -C codegen-units=1 option to the RUSTFLAGS environment variable. This reduces the size of the toolchain by ~5.5% while increasing performance by ~1.8%.
>
> — [android-developers.googleblog.com/2023/12/faster-rust-toolchains-fo…](https://android-developers.googleblog.com/2023/12/faster-rust-toolchains-for-android.html)

> Be aware that setting this option will slow down the time it takes to build the toolchain by ~2x (measured on our workstations).
>
> — [android-developers.googleblog.com/2023/12/faster-rust-toolchains-fo…](https://android-developers.googleblog.com/2023/12/faster-rust-toolchains-for-android.html)

</details>
