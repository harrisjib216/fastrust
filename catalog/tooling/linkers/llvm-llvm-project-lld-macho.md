# LLVM lld for macOS (ld64.lld)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md) › [Linkers](README.md)</sub>

**Tier B** (independent measurement) · ❌ Doesn't hold up · Open source · `C++` · `Apache-2.0 WITH LLVM-exception` · 🟢 active

> LLVM's Mach-O linker: once 4x faster than Apple's ld64, now slower than Apple's ld-prime default on Apple silicon.

| | |
|---|---|
| **Beats** | Apple ld-prime, the default linker with Xcode 15 or later, called through cc |
| **Claimed result** | Historic (older Chromium doc): lld 4x faster than ld64 linking Chromium Framework. Current: switching from lld to ld-prime gives up to a 4-6x speedup for local arm64 macOS Chromium builds |
| **Workload** | Linking Chromium on macOS: local non-cross arm64 developer builds (current doc); Chromium Framework in symbol_level=0 release builds (2021 doc) |
| **Metric** | time (link) |
| **Hardware** | Apple silicon (arm64) Macs; models not stated |
| **OS** | macOS |
| **Toolchain** | Chromium's LLD from its clang package vs Apple ld-prime from Xcode |
| **Adoption** | 41.2k stars |
| **Last commit** | 2026-10-29 |
| **Platforms** | Mach-O port of lld; lld also covers ELF, COFF and Wasm |
| **Install** | brew install llvm, then -C link-arg=-fuse-ld=<path>/ld64.lld |
| **Links** | [repository](https://github.com/llvm/llvm-project) · [documentation](https://lld.llvm.org/MachO/index.html) · [benchmark](https://chromium.googlesource.com/chromium/src.git/+/main/docs/apple_platform_linkers.md) |

## Verdict

An older Chromium doc (2021 or earlier) reported lld 4x faster than Apple's old ld64, but Apple replaced ld64 with ld-prime in Xcode 15. Chromium's current docs say ld-prime is faster than lld for local arm64 macOS builds, by up to 4-6x, and Bevy's build config now keeps the default macOS linker for the same reason.

**Why tier B:** Chromium's build team measured on its own codebase; it is independent of Apple and contributes to LLD, so a result in ld-prime's favour is not self-serving. Detailed timings are in a bug tracker, not the doc.

## Use it when

- Cross-compiling to macOS from Linux, where Apple's linker is not available
- Builds that need one linker across platforms or LTO with matching LLVM versions (Chromium's reasons for keeping LLD)

## Skip it when

- Native arm64 macOS development on Xcode 15 or later: the default linker is faster

## How it is fast

lld's Mach-O port follows the same design as its ELF port, which beat Apple's older ld64 by a wide margin. Apple's rewritten linker, ld-prime, shipped with Xcode 15 and now leads on arm64 Macs, so the speed case for switching has reversed.

## What it costs

- Extra install of LLVM
- Slower than the default linker on Apple silicon

## Problems with the evidence

- Chromium's doc gives a range ('up to 4-6x') without the underlying timings
- Chromium is a very large C++ program; Rust binaries are usually smaller, so absolute savings are smaller
- The doc does not date the benchmark

## Evidence

<details>
<summary>4 quotes from the sources, checked 2026-10-28</summary>

> Recent benchmarks show that Apple's new linker (ld-prime) is faster than LLD for local macOS development
>
> — [chromium.googlesource.com/chromium/src.git/+/main/docs/apple_platfo…](https://chromium.googlesource.com/chromium/src.git/+/main/docs/apple_platform_linkers.md)

> Supported only for local non-cross arm64 macOS builds (ARM Mac) to improve link speed (yielding up to a 4-6x speedup depending on configuration).
>
> — [chromium.googlesource.com/chromium/src.git/+/main/docs/apple_platfo…](https://chromium.googlesource.com/chromium/src.git/+/main/docs/apple_platform_linkers.md)

> LLD is currently 4x faster at linking Chromium Framework than ld64
>
> — [chromium.googlesource.com/chromium/src/+/da492f144feba8874ce6d25a9d…](https://chromium.googlesource.com/chromium/src/+/da492f144feba8874ce6d25a9da170228763b21e/docs/mac_lld.md)

> On macOS, the default linker yields higher performance than LLD and is used instead.
>
> — [github.com/bevyengine/bevy/blob/main/.cargo/config_fast_builds.toml](https://github.com/bevyengine/bevy/blob/main/.cargo/config_fast_builds.toml)

</details>
