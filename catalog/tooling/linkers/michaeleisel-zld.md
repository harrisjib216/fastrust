# zld (faster fork of Apple ld64)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md) › [Linkers](README.md)</sub>

**Tier D** (directional only) · ❌ Doesn't hold up · Open source · `C++` · `MIT` · 🔴 abandoned

> Archived fork of Apple's old ld64 that claimed links at least 40% faster; Xcode 15's ld-prime replaced the linker it improved on.

| | |
|---|---|
| **Beats** | Apple's default macOS linker called through cc: ld-prime with Xcode 15 or later, ld64 when zld was current |
| **Claimed result** | Linking at least 40% faster than Apple's linker (README text; full results in a chart) |
| **Workload** | Not stated in text; the README says zld is meant for debug builds, and the results are in an SVG chart |
| **Metric** | time (link) |
| **OS** | macOS |
| **Adoption** | 1.2k stars |
| **Last commit** | 2023-02-15 |
| **Platforms** | macOS; required a full Xcode install |
| **Install** | Archived; old prebuilt binaries on the GitHub releases page |
| **Links** | [repository](https://github.com/michaeleisel/zld) · [benchmark](https://github.com/michaeleisel/zld/blob/master/README.md) |

## Verdict

zld claimed links at least 40% faster than ld64, but it was archived in February 2023 and its author points to lld instead. Apple replaced ld64 with ld-prime in Xcode 15, and Chromium now measures ld-prime as faster than lld, so the baseline zld beat is gone.

**Why tier D:** The only number in text is a floor ('at least 40% faster'); the results themselves are an SVG chart.

## Use it when

- None for new work

## Skip it when

- Any build on Xcode 15 or later, where the default ld-prime replaced the ld64 baseline
- Projects that need a maintained linker

## How it is fast

zld forked the last open-source release of Apple's ld64 and changed its hashing code for speed, keeping output otherwise byte-for-byte identical. It also kept caches between runs, so repeat links were faster than the first.

## What it costs

- Archived and tied to an old ld64 version; may break with newer Xcode

## Problems with the evidence

- Numbers are a chart image plus a 'not at least 40% faster' floor in text
- Measured against ld64, which is no longer the default

## Evidence

<details>
<summary>5 quotes from the sources, checked 2026-10-28</summary>

> ## NOTE: zld is now archived, consider using lld instead.
>
> — [github.com/michaeleisel/zld/blob/master/README.md](https://github.com/michaeleisel/zld/blob/master/README.md)

> ## A faster version of Apple's linker
>
> — [github.com/michaeleisel/zld/blob/master/README.md](https://github.com/michaeleisel/zld/blob/master/README.md)

> Feel free to file an issue if you find that linking is not at least 40% faster for your case
>
> — [github.com/michaeleisel/zld/blob/master/README.md](https://github.com/michaeleisel/zld/blob/master/README.md)

> Without a few optimizations around hashing, it would produce byte-for-byte the same executables as the open-source one.
>
> — [github.com/michaeleisel/zld/blob/master/README.md](https://github.com/michaeleisel/zld/blob/master/README.md)

> Recent benchmarks show that Apple's new linker (ld-prime) is faster than LLD for local macOS development
>
> — [chromium.googlesource.com/chromium/src.git/+/main/docs/apple_platfo…](https://chromium.googlesource.com/chromium/src.git/+/main/docs/apple_platform_linkers.md)

</details>
