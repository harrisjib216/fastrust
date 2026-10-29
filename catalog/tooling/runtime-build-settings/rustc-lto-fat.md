# Fat LTO (lto = "fat") in the release profile

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md) › [Build settings for faster binaries](README.md)</sub>

**Tier B** (independent measurement) · 🟡 Holds up in part · Ships with Rust · `Rust` · 🟢 active

> Whole-program LTO across all crates: single-digit runtime gains in the best measurement, at several times the release build time.

| | |
|---|---|
| **Beats** | cargo build --release with the defaults lto = false (thin-local LTO) and codegen-units = 16 |
| **Claimed result** | Rust Performance Book: LTO can improve runtime by 10-20% or more. Android rustc: ~6% from thin LTO over the previous stage (~7.7% over baseline with codegen-units=1 and section GC). Ruff incremental release rebuild: 77s with its fat-LTO profile, 41s with thin LTO, 16s with Cargo's default release settings |
| **Workload** | Android: time to build Android's Rust components with each toolchain variant. Ruff: release rebuild of ruff_cli after touching one source file |
| **Metric** | time; build time |
| **Toolchain** | Android: thin LTO over the Rust and C++ code of the compiler; Ruff: release profile variants |
| **Measured** | 2023-12 |
| **Platforms** | All targets |
| **Install** | [profile.release] lto = "fat" (or lto = "thin") |
| **Links** | [documentation](https://doc.rust-lang.org/cargo/reference/profiles.html) · [benchmark](https://android-developers.googleblog.com/2023/12/faster-rust-toolchains-for-android.html) |

## Verdict

The usual '10-20% or more' figure comes without data. The best-documented measurement, Android's rustc toolchain, got about 6% from thin LTO; Ruff's fat-LTO release rebuild took 77 s against 16 s with Cargo's defaults, and Ruff calls the runtime difference slight. Cargo's docs say thin LTO gets similar gains to fat in much less time.

**Why tier B:** Android's figure is an independent measurement on a large program; Ruff's build times are one ad hoc run; the Rust Performance Book range has no linked data.

## Use it when

- Final release builds where a few percent matters and CI time is cheap
- Reducing binary size

## Skip it when

- Iterative release builds or benchmarking loops: Ruff calls fat-LTO compile times untenable for that
- Memory-limited build machines

## How it is fast

LTO defers optimization to link time, when LLVM can see code from every crate and inline or remove code across crate boundaries. Fat LTO merges everything into one module and optimizes it mostly serially; thin LTO works from summaries and optimizes in parallel, which is why Cargo says it costs much less time for similar gains.

## What it costs

- Several times longer release builds
- Higher memory use while linking
- Smaller binaries

## Problems with the evidence

- The 10-20% figure in the Rust Performance Book has no linked data
- Android measured thin LTO, not fat, on a mixed C++/Rust program
- Ruff's timings are a single incremental run, and its old profile also differed in codegen-units

## Evidence

<details>
<summary>9 quotes from the sources, checked 2026-10-28</summary>

> Link-time optimization (LTO) is a whole-program optimization technique that can improve runtime speed by 10-20% or more, and also reduce binary size, at the cost of worse compile times.
>
> — [nnethercote.github.io/perf-book/build-configuration.html](https://nnethercote.github.io/perf-book/build-configuration.html)

> Using CGU1, GC sections, and LTO produces a speedup of ~7.7% and size improvement of ~5.4% over the baseline. This works out to a speedup of ~6% over the previous stage in the pipeline due solely to LTO.
>
> — [android-developers.googleblog.com/2023/12/faster-rust-toolchains-fo…](https://android-developers.googleblog.com/2023/12/faster-rust-toolchains-for-android.html)

> With status quo release profile, build takes 77s
>
> — [github.com/astral-sh/ruff/pull/9031](https://github.com/astral-sh/ruff/pull/9031)

> build takes 41s
>
> — [github.com/astral-sh/ruff/pull/9031](https://github.com/astral-sh/ruff/pull/9031)

> build takes 16s (i believe this is the default
>
> — [github.com/astral-sh/ruff/pull/9031](https://github.com/astral-sh/ruff/pull/9031)

> instead of using "fat" LTO, and sets
>
> — [github.com/astral-sh/ruff/pull/9031](https://github.com/astral-sh/ruff/pull/9031)

> But compile times with `lto = fat` are completely untenable.
>
> — [github.com/astral-sh/ruff/blob/main/Cargo.toml](https://github.com/astral-sh/ruff/blob/main/Cargo.toml)

> a hit to binary size and a slight hit to runtime performance in our
>
> — [github.com/astral-sh/ruff/blob/main/Cargo.toml](https://github.com/astral-sh/ruff/blob/main/Cargo.toml)

> This is similar to “fat”, but takes substantially less time to run while still achieving performance gains similar to “fat”.
>
> — [doc.rust-lang.org/cargo/reference/profiles.html](https://doc.rust-lang.org/cargo/reference/profiles.html)

</details>
