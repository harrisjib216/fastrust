# mold (ELF linker)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md) › [Linkers](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Open source · `Rust` · `MIT` · 🟢 active

> Parallel ELF linker by lld's original author; links large programs several times faster than lld and far faster than GNU ld.

| | |
|---|---|
| **Beats** | rust-lld, the default linker on x86_64 Linux since Rust 1.90; GNU ld through cc on other Linux targets |
| **Claimed result** | 4.9x faster than lld at the median (August 2026 suite). Threadripper 7980X debug links: Chromium 145 1.65s vs 16.64s for lld (10.1x), TensorFlow 2.21 3.15s vs 50.73s (16.1x). M1 Ultra: Clang 21 debug 2.96s vs 4.40s (1.5x). Third-party Rust warm rebuild: cargo run 7.539 s with mold vs 20.202 s with GNU ld; 1.156 s with mold vs 1.602 s with lld in the author's final configuration |
| **Workload** | Linking nine large C/C++ programs (Blender, Chromium, Clang, ClickHouse, Firefox, Godot, LibreOffice, PyTorch, TensorFlow) in debug and release builds, median of three runs after a warm-up. Third party: warm cargo run after a trivial edit of a crate with heavy dependencies. |
| **Metric** | time (link wall-clock); build time |
| **Hardware** | AMD Ryzen Threadripper 7980X (64 cores); Apple M1 Ultra (performance cores only); third party: Intel i7-10510U laptop |
| **OS** | Ubuntu 24.04; Fedora Asahi Remix 42; third party: Pop!_OS Linux |
| **Toolchain** | lld, wild and mold built from source in release configuration as of 2026-08-28, default options; third party: mold 2.3.3 with clang-15 as the linker driver |
| **Measured** | 2026-08 |
| **Adoption** | 17.7k stars |
| **Last commit** | 2026-10-22 |
| **Platforms** | ELF targets on Linux: x86-64, i386, ARM 32/64, RISC-V 32/64, PowerPC 32/64, s390x, LoongArch, SPARC64, m68k, SH-4; no macOS or Windows |
| **Install** | Distro package (apt/dnf/pacman install mold) or a GitHub release binary; then rustflags = ["-C", "link-arg=-fuse-ld=mold"] in .cargo/config.toml |
| **Links** | [repository](https://github.com/rui314/mold) · [benchmark](https://github.com/rui314/mold/blob/main/README.md) |

## Verdict

mold's public benchmark suite shows it linking 4.9x faster than lld at the median, and a third-party Rust test cut a warm rebuild from 20.2 s with GNU ld to 7.5 s. The end-to-end gain over lld in a Rust build is much smaller than the link-only ratio, because linking is only part of the build.

**Why tier A:** The benchmark inputs are published on Zenodo, the linkers were built from source on a stated date with default options, and hardware and OS are stated; a separate third-party Rust measurement agrees on direction.

## Use it when

- Large binaries or heavy debug info on Linux, where linking is a big part of rebuild time
- Many-core workstations, where the measured gap over lld is widest
- Linux targets other than x86_64, where Rust still defaults to the system GNU ld

## Skip it when

- macOS or Windows: mold links ELF only
- Small programs whose link step already takes well under a second
- Builds relying on unusual linker scripts or flags; test before switching CI

## How it is fast

mold was written from scratch by the original developer of lld to get past the limits he hit while optimizing lld. The README credits pervasive parallelism plus efficient data structures and algorithms, with details in the authors' paper. Its lead grows with core count: the gap over lld is wider on the 64-core Threadripper than on the M1 Ultra.

## What it costs

- An extra tool to install and a linker flag in .cargo/config.toml
- Uses all cores while linking
- Not every exotic linker feature is guaranteed to match GNU ld

## Problems with the evidence

- The lld comparison is first-party and uses C/C++ programs, not Rust crates
- Link-only ratios overstate end-to-end gains: in the third-party Rust test, mold vs lld was 1.156 s vs 1.602 s for a whole warm cargo run (1.39x, derived from page data)
- The GNU ld comparison is from February 2024 with mold 2.3.3; x86_64 Linux has since moved to lld by default

## Evidence

<details>
<summary>10 quotes from the sources, checked 2026-10-28</summary>

> In our August 2026 benchmarks, it links 4.9x faster than [LLVM lld](https://lld.llvm.org) and 1.9x faster than [wild](https://github.com/wild-linker/wild) at the median
>
> — [github.com/rui314/mold/blob/main/README.md](https://github.com/rui314/mold/blob/main/README.md)

> | Chromium 145 (4.51 GiB)           | 16.64s | 3.98s | 1.65s | 2.4x      | 10.1x
>
> — [github.com/rui314/mold/blob/main/README.md](https://github.com/rui314/mold/blob/main/README.md)

> | TensorFlow 2.21 (9.55 GiB)        | 50.73s | N/A   | 3.15s | N/A       | 16.1x
>
> — [github.com/rui314/mold/blob/main/README.md](https://github.com/rui314/mold/blob/main/README.md)

> | Clang 21 (4.19 GiB)               | 4.40s  | 2.78s | 2.96s | 0.9x      | 1.5x
>
> — [github.com/rui314/mold/blob/main/README.md](https://github.com/rui314/mold/blob/main/README.md)

> All three linkers were built from source in release configuration as of 2026-08-28 and ran with their default options.
>
> — [github.com/rui314/mold/blob/main/README.md](https://github.com/rui314/mold/blob/main/README.md)

> AMD Ryzen Threadripper 7980X (64 cores) running Ubuntu 24.04
>
> — [github.com/rui314/mold/blob/main/README.md](https://github.com/rui314/mold/blob/main/README.md)

> To start, we benchmark the cargo run time for a default configuration debug build using the default system linker (GNU ld). This gives us a time of 20.202 s ± 0.256 s which is slow enough to be fairly annoying.
>
> — [davidlattimore.github.io/posts/2024/02/04/speeding-up-the-rust-edit…](https://davidlattimore.github.io/posts/2024/02/04/speeding-up-the-rust-edit-build-run-cycle.html)

> That reduces our cargo run time to 7.539 s ± 1.691 s.
>
> — [davidlattimore.github.io/posts/2024/02/04/speeding-up-the-rust-edit…](https://davidlattimore.github.io/posts/2024/02/04/speeding-up-the-rust-edit-build-run-cycle.html)

> This reduces our cargo run time to 1.156 s ± 0.039 s.
>
> — [davidlattimore.github.io/posts/2024/02/04/speeding-up-the-rust-edit…](https://davidlattimore.github.io/posts/2024/02/04/speeding-up-the-rust-edit-build-run-cycle.html)

> Switching to lld increases our cargo run time to 1.602 s ± 0.011 s, which is still pretty good.
>
> — [davidlattimore.github.io/posts/2024/02/04/speeding-up-the-rust-edit…](https://davidlattimore.github.io/posts/2024/02/04/speeding-up-the-rust-edit-build-run-cycle.html)

</details>
