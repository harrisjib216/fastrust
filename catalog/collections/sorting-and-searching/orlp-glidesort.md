# glidesort (stable hybrid merge/quick sort)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Sorting and searching](README.md)</sub>

**Tier D** (directional only) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🔴 abandoned

> Stable sort mixing powersort run merging with stable quicksort; its ideas now ship in std's driftsort, which narrows the gap.

| | |
|---|---|
| **Beats** | slice::sort and sort_unstable as of rustc 1.69 nightly (before driftsort and ipnsort) |
| **Claimed result** | Chart only (Apple M1, u64, rustc 1.69.0-nightly). Binary size per u64 instantiation: 25520 bytes for glidesort vs 5294 for driftsort and 2335 for the previous std slice::sort; about 13x the old sort, against about 2.5x for driftsort. |
| **Workload** | Sorting u64 slices across input distributions (chart); binary size of one instantiation (writeup). |
| **Metric** | time (chart); binary size |
| **Hardware** | Apple M1 (2021) |
| **Toolchain** | rustc 1.69.0-nightly (11d96b593), --release --features unstable, lto = "thin" |
| **Measured** | 2023-02 |
| **Adoption** | 1.7k stars |
| **Last commit** | 2023-02-06 |
| **Platforms** | All Rust targets |
| **Install** | cargo add glidesort |
| **Links** | [repository](https://github.com/orlp/glidesort) · [documentation](https://docs.rs/glidesort) · [benchmark](https://github.com/orlp/glidesort) |

## Verdict

glidesort's published lead was measured against the pre-1.81 std sorts, and its own numbers are a chart image. std's driftsort (1.81) applies the same ideas; the driftsort writeup finds the two close on most patterns, with glidesort ahead only on some partially sorted and low-cardinality inputs, at about 5x driftsort's binary size.

**Why tier D:** First-party results are only in a chart image; the only text numbers are binary-size figures from the driftsort writeup.

## Use it when

- You are pinned to a Rust toolchain older than 1.81 and need a faster stable sort.
- You need sorting with a caller-provided buffer (sort_with_buffer) or bounded auxiliary memory.

## Skip it when

- On Rust 1.81 or later for general use: std's slice::sort is close on most patterns and much smaller.
- Binary size or compile time matters.

## How it is fast

glidesort scans for sorted runs like powersort but defers sorting unsorted stretches, merging logical runs and sorting them with a branchless stable quicksort only when needed. It interleaves several merges for memory- and instruction-level parallelism.

## What it costs

- Large code size per instantiation (25520 bytes for u64 in release) and longer compile times, per the driftsort writeup.

## Problems with the evidence

- First-party benchmark exists only as a chart image, against std sorts that have since been replaced.
- No commits since February 2023.

## Evidence

<details>
<summary>4 quotes from the sources, checked 2026-10-28</summary>

> Compiled with `rustc 1.69.0-nightly (11d96b593)` using `--release --features unstable` and `lto = "thin"`.
>
> — [github.com/orlp/glidesort/blob/master/README.md](https://github.com/orlp/glidesort/blob/master/README.md)

> driftsort is similar in terms of effective capabilities to glidesort while only requiring ~2.5x the binary-size in contrast to ~13x for glidesort.
>
> — [github.com/Voultapher/sort-research-rs/blob/main/writeup/driftsort_…](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/driftsort_introduction/text.md)

> release                      | `u64`    | 2335                 | 25520                  | 5294
>
> — [github.com/Voultapher/sort-research-rs/blob/main/writeup/driftsort_…](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/driftsort_introduction/text.md)

> random_p5 sees the only major difference between driftsort and glidesort at this input length.
>
> — [github.com/Voultapher/sort-research-rs/blob/main/writeup/driftsort_…](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/driftsort_introduction/text.md)

</details>
