# wild (ELF linker)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md) › [Linkers](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Young Rust-written ELF linker aimed at fast iterative builds; beats lld where it works but trails mold and cannot link some large programs.

| | |
|---|---|
| **Beats** | rust-lld, the default linker on x86_64 Linux since Rust 1.90 |
| **Claimed result** | Threadripper 7980X debug links (mold's August 2026 suite): Chromium 145 3.98s vs 16.64s for lld, Clang 21 3.71s vs 6.19s; mold was faster still (1.65s, 1.34s). M1 Ultra Clang 21 debug: wild 2.78s vs lld 4.40s and mold 2.96s. wild could not link Firefox 149, TensorFlow 2.21 or Chromium 145 ARM64 (N/A) |
| **Workload** | Linking nine large C/C++ programs in debug and release builds, median of three runs after a warm-up |
| **Metric** | time (link wall-clock) |
| **Hardware** | AMD Ryzen Threadripper 7980X (64 cores); Apple M1 Ultra (performance cores only) |
| **OS** | Ubuntu 24.04; Fedora Asahi Remix 42 |
| **Toolchain** | wild, lld and mold built from source in release configuration as of 2026-08-28, default options |
| **Measured** | 2026-08 |
| **Adoption** | 4.1k stars |
| **Last commit** | 2026-10-29 |
| **Platforms** | Linux on x86-64, AArch64, RISC-V (riscv64gc), LoongArch64, PPC64LE (initial); WebAssembly; no Mach-O or Windows yet |
| **Install** | cargo binstall wild-linker (or cargo install --locked wild-linker) |
| **Links** | [repository](https://github.com/wild-linker/wild) · [documentation](https://github.com/wild-linker/wild/blob/main/USAGE.md) · [benchmark](https://github.com/rui314/mold/blob/main/README.md) |

## Verdict

In mold's public suite, wild linked every program it could handle faster than lld, 1.5x to 4.2x on the Threadripper debug links (derived from page data). But it could not link Firefox, TensorFlow or the ARM64 Chromium build, mold was 1.9x faster at the median, and the incremental linking wild is designed around is not implemented yet.

**Why tier A:** The numbers come from mold's reproducible suite (inputs on Zenodo, hardware and build settings stated), which is independent of wild's authors; wild's own benchmark pages give charts only.

## Use it when

- Linux development builds where you want a faster link than the lld default and can fall back if a link fails
- Rust projects willing to track a fast-moving linker in exchange for future incremental linking

## Skip it when

- You need the fastest non-incremental ELF linker today: mold was faster on most rows of the same suite
- macOS or Windows
- Release builds that need identical code folding: wild does not implement --icf=all

## How it is fast

wild is a from-scratch linker written in Rust with the stated goal of being very fast for iterative development. Its planned main mechanism, incremental linking, is not implemented yet, so today's results come from its non-incremental design; the README and FAQ do not describe its internals further.

## What it costs

- Less mature than lld or mold; some large programs fail to link
- Missing features such as --icf=all

## Problems with the evidence

- The comparison was run by mold's author, a competitor, though the suite is public
- Programs are C/C++, not Rust
- wild's own benchmark pages (which include rust-analyzer, Zed and Bevy) publish charts only

## Evidence

<details>
<summary>11 quotes from the sources, checked 2026-10-28</summary>

> | Chromium 145 (4.51 GiB)           | 16.64s | 3.98s | 1.65s | 2.4x      | 10.1x
>
> — [github.com/rui314/mold/blob/main/README.md](https://github.com/rui314/mold/blob/main/README.md)

> | Clang 21 (4.19 GiB)               | 6.19s  | 3.71s | 1.34s | 2.8x      | 4.6x
>
> — [github.com/rui314/mold/blob/main/README.md](https://github.com/rui314/mold/blob/main/README.md)

> | Firefox 149 (2.37 GiB)            | 5.11s  | N/A   | 0.78s | N/A       | 6.5x
>
> — [github.com/rui314/mold/blob/main/README.md](https://github.com/rui314/mold/blob/main/README.md)

> | Clang 21 (4.19 GiB)               | 4.40s  | 2.78s | 2.96s | 0.9x      | 1.5x
>
> — [github.com/rui314/mold/blob/main/README.md](https://github.com/rui314/mold/blob/main/README.md)

> | TensorFlow 2.21 (9.55 GiB)        | 50.73s | N/A   | 3.15s | N/A       | 16.1x
>
> — [github.com/rui314/mold/blob/main/README.md](https://github.com/rui314/mold/blob/main/README.md)

> N/A indicates that the linker cannot link that program.
>
> — [github.com/rui314/mold/blob/main/README.md](https://github.com/rui314/mold/blob/main/README.md)

> wild does not implement `--icf=all`
>
> — [github.com/rui314/mold/blob/main/README.md](https://github.com/rui314/mold/blob/main/README.md)

> All three linkers were built from source in release configuration as of 2026-08-28 and ran with their default options.
>
> — [github.com/rui314/mold/blob/main/README.md](https://github.com/rui314/mold/blob/main/README.md)

> AMD Ryzen Threadripper 7980X (64 cores) running Ubuntu 24.04
>
> — [github.com/rui314/mold/blob/main/README.md](https://github.com/rui314/mold/blob/main/README.md)

> The plan is to eventually make it incremental, however that isn't yet implemented.
>
> — [github.com/wild-linker/wild/blob/main/README.md](https://github.com/wild-linker/wild/blob/main/README.md)

> Wild doesn't do incremental linking yet, but that is the end-goal.
>
> — [github.com/wild-linker/wild/blob/main/FAQ.md](https://github.com/wild-linker/wild/blob/main/FAQ.md)

</details>
