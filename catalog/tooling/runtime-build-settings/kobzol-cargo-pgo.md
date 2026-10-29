# cargo-pgo (profile-guided optimization)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md) › [Build settings for faster binaries](README.md)</sub>

**Tier B** (independent measurement) · ✅ Holds up · Open source · `Rust` · `MIT` · 🟢 active

> Cargo subcommand that automates profile-guided optimization; PGO made rustc itself 10-20% faster in several measurements.

| | |
|---|---|
| **Beats** | cargo build --release with no profile data |
| **Claimed result** | PGO-built rustc: 10-16% shorter build times on real-world crates (rust-lang experiment, Ryzen 1700X); Windows rustc: 10-20% (Rust 1.64.0); Android rustc: 13.2% faster than the LTO-only toolchain; Rust Performance Book: 10% or more |
| **Workload** | The Rust compiler as the optimized program, trained on and timed with real-world crate builds |
| **Metric** | time |
| **Hardware** | Ryzen 1700X (2020 rust-lang experiment) |
| **OS** | Linux, Windows |
| **Toolchain** | rustc -Cprofile-generate / -Cprofile-use with llvm-profdata |
| **Measured** | 2023-12 |
| **Adoption** | 793 stars |
| **Last commit** | 2026-04-25 |
| **Platforms** | Targets where rustc supports PGO; needs llvm-profdata from the llvm-tools-preview component |
| **Install** | cargo install cargo-pgo; rustup component add llvm-tools-preview |
| **Links** | [repository](https://github.com/kobzol/cargo-pgo) · [documentation](https://doc.rust-lang.org/rustc/profile-guided-optimization.html) · [benchmark](https://blog.rust-lang.org/inside-rust/2020/11/11/exploring-pgo-for-the-rust-compiler/) |

## Verdict

PGO on a large Rust program has repeated, independent results: 10-16% shorter rustc build times in the rust-lang experiment, 10-20% for Windows rustc, and 13.2% on top of LTO in Google's Android toolchain. cargo-pgo drives the same rustc flags; the gain depends on how representative the training run is.

**Why tier B:** The numbers come from the rust-lang project and Google applying PGO to rustc, not from cargo-pgo's author; cargo-pgo's README has no benchmarks.

## Use it when

- Large, branchy programs (compilers, interpreters, servers, parsers) with a workload you can reproduce
- Binaries you build and distribute yourself

## Skip it when

- Crates installed with cargo install: the Rust Performance Book notes PGO is not supported there
- Workloads that differ a lot from the training run

## How it is fast

PGO builds an instrumented binary, runs it on representative input to count how often each branch and function runs, then rebuilds with that profile. LLVM uses the counts to inline hot calls, place hot code together and order branches by likelihood, which cuts instruction-cache misses and branch mispredictions.

## What it costs

- Two builds and a training run per release
- Profile files to manage in CI

## Problems with the evidence

- All quoted numbers are for rustc; other programs will differ
- The 2020 numbers come from one machine
- Profiles must be regenerated as code changes

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> As you can see we get a 10-16% reduction of build times almost across the board for real world test cases.
>
> — [blog.rust-lang.org/inside-rust/2020/11/11/exploring-pgo-for-the-rus…](https://blog.rust-lang.org/inside-rust/2020/11/11/exploring-pgo-for-the-rust-compiler/)

> It's possible that the Ryzen 1700X processor I used has some idiosyncrasies that favor the kind of optimizations that PGO does
>
> — [blog.rust-lang.org/inside-rust/2020/11/11/exploring-pgo-for-the-rus…](https://blog.rust-lang.org/inside-rust/2020/11/11/exploring-pgo-for-the-rust-compiler/)

> Windows builds now use profile-guided optimization, providing 10-20% improvements to compiler performance:
>
> — [github.com/rust-lang/rust/blob/main/RELEASES.md](https://github.com/rust-lang/rust/blob/main/RELEASES.md)

> Version 1.64.0 (2022-09-22)
>
> — [github.com/rust-lang/rust/blob/main/RELEASES.md](https://github.com/rust-lang/rust/blob/main/RELEASES.md)

> As a result, the toolchain achieves a ~19.8% speedup and 5.3% reduction in size over the baseline compiler. This is a 13.2% speedup over the previous stage in the compiler.
>
> — [android-developers.googleblog.com/2023/12/faster-rust-toolchains-fo…](https://android-developers.googleblog.com/2023/12/faster-rust-toolchains-for-android.html)

> This can improve runtime speed by 10% or more.
>
> — [nnethercote.github.io/perf-book/build-configuration.html](https://nnethercote.github.io/perf-book/build-configuration.html)

> Unfortunately, PGO is not supported for binaries hosted on crates.io and distributed via
>
> — [nnethercote.github.io/perf-book/build-configuration.html](https://nnethercote.github.io/perf-book/build-configuration.html)

</details>
