# rust-lld as the default linker on x86_64 Linux (Rust 1.90)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md) › [Linkers](README.md)</sub>

**Tier B** (independent measurement) · ✅ Holds up · Ships with Rust · `C++` · 🟢 active

> Since Rust 1.90, rustc links x86_64 Linux programs with its bundled lld instead of GNU ld, which cuts link time several-fold.

| | |
|---|---|
| **Beats** | GNU ld (BFD) called through cc, the default linker for x86_64-unknown-linux-gnu before Rust 1.90 |
| **Claimed result** | ripgrep debug build: linking 7x faster and end-to-end compile time 40% lower on an incremental rebuild; 20% lower on a from-scratch debug build |
| **Workload** | ripgrep 13 built in debug mode on Linux: an incremental rebuild and a from-scratch build. The post links results for the full rustc benchmark suite. |
| **Metric** | build time |
| **OS** | Linux |
| **Toolchain** | rustc nightly and 1.90.0 beta on x86_64-unknown-linux-gnu; rust-lld compared with the system linker called through cc |
| **Measured** | 2025-09 |
| **Platforms** | x86_64-unknown-linux-gnu only; other Linux targets still call the system linker by default |
| **Install** | Default since Rust 1.90.0 on x86_64-unknown-linux-gnu; opt out with -C linker-features=-lld |
| **Links** | [documentation](https://blog.rust-lang.org/2025/09/18/Rust-1.90.0/) · [benchmark](https://blog.rust-lang.org/2025/09/01/rust-lld-on-1.90.0-stable/) |

## Verdict

The Rust project measured ripgrep's debug build: 7x less link time and 40% less end-to-end time on an incremental rebuild, and 20% less on a from-scratch build. The switch shipped in Rust 1.90.0 (2025-09-18), so x86_64 Linux users already have it.

**Why tier B:** The rust-lang compiler performance working group measured its own toolchain change; the post quotes one project and links the full benchmark results.

## Use it when

- You build on x86_64 Linux with Rust 1.90 or later: it is already on and needs no configuration
- Incremental debug rebuilds, where linking is a large share of the time

## Skip it when

- A crate depends on GNU ld-specific behaviour; revert with -C linker-features=-lld
- Resource-constrained build machines, where the linker's parallelism competes with other work

## How it is fast

GNU ld dates from single-core machines and does most of its work on one thread. lld does much of its work in parallel and was built for link speed; the Rust post says some of the biggest gains come from that parallelism. The gain is largest for big binaries, binaries with a lot of debug info, and incremental rebuilds, where the linker is most of the remaining work.

## What it costs

- lld is not bug-for-bug compatible with GNU ld
- Uses more threads during linking

## Problems with the evidence

- The post quotes numbers for one project (ripgrep); the rest are on a linked results page
- Hardware for the ripgrep numbers is not stated

## Evidence

<details>
<summary>3 quotes from the sources, checked 2026-10-28</summary>

> Here are more details from the ripgrep example mentioned above: for an incremental rebuild, linking is reduced 7x, resulting in a 40% reduction in end-to-end compilation times. For a from-scratch debug build, it is a 20% improvement.
>
> — [blog.rust-lang.org/2025/09/01/rust-lld-on-1.90.0-stable/](https://blog.rust-lang.org/2025/09/01/rust-lld-on-1.90.0-stable/)

> Some of the big gains in performance come from parallelism, which could be undesirable in resource-constrained environments, or for heavy projects that are already reaching hardware limits.
>
> — [blog.rust-lang.org/2025/09/01/rust-lld-on-1.90.0-stable/](https://blog.rust-lang.org/2025/09/01/rust-lld-on-1.90.0-stable/)

> The x86_64-unknown-linux-gnu target will now use the LLD linker for linking Rust crates by default.
>
> — [blog.rust-lang.org/2025/09/18/Rust-1.90.0/](https://blog.rust-lang.org/2025/09/18/Rust-1.90.0/)

</details>
