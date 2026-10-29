# std slice::sort (driftsort, Rust 1.81)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Sorting and searching](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Ships with Rust · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Since Rust 1.81, slice::sort is driftsort: about 2.4x faster on random u64 and far faster on inputs with many duplicates.

| | |
|---|---|
| **Beats** | slice::sort before Rust 1.81 (TimSort-style merge sort) |
| **Claimed result** | Random u64: about 2.4x throughput for N > 1e4 and <= 1e6; random_d20 and random_p5 patterns: about 13x and 17x at 1e6 elements (Zen 3). Binary size per u64 instantiation 2335 -> 5294 bytes (release). |
| **Workload** | Stable sort of u64, i32, String, a 1 KiB struct and f128-like floats across patterns (random, low-cardinality, partially sorted, ascending, descending), lengths up to 1e7. |
| **Metric** | throughput |
| **Hardware** | AMD Ryzen 9 5900X (Zen 3); Intel i7-5500U; Apple M1 Pro |
| **OS** | Linux 6.6 (Zen 3); Linux 5.19; macOS (Darwin 22.6.0) |
| **Toolchain** | rustc 1.77.0-nightly (6ae4cfbbb 2024-01-17) |
| **Measured** | 2024-01 |
| **Platforms** | All targets |
| **Install** | built into std since 1.81 |
| **Links** | [documentation](https://doc.rust-lang.org/std/primitive.slice.html#method.sort) · [benchmark](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/driftsort_introduction/text.md) |

## Verdict

On Zen 3, driftsort sorts random u64 inputs about 2.4x faster than the old stable sort for 10^4 to 10^6 elements, and low-cardinality inputs up to 13x to 17x faster. It is std's stable sort since 1.81, so third-party stable sorts benchmarked against older std overstate their lead.

**Why tier A:** Public benchmark code (sort-research-rs) with CPU, OS and rustc version stated for three machines; the authors also wrote the std change.

## Use it when

- Default stable sorting on any type; there is little left to gain from third-party comparison sorts.

## Skip it when

- Binary size matters more than speed (embedded, Wasm): each instantiation is larger; the authors point to tiny_sort for that case.
- Large integer arrays where a radix sort applies.

## How it is fast

driftsort combines run detection from merge sort with a stable quicksort for unsorted regions, so already-sorted runs are kept and the rest is partitioned. Remembering earlier pivots lets it filter out values that repeat, which turns low-cardinality inputs into near-linear work. A small-sort tuned for superscalar CPUs replaces the old insertion-sort-heavy inner loop.

## What it costs

- Binary size grows (2335 -> 5294 bytes per u64 instantiation in release).
- Allocates a buffer of N elements up to 8 MB, then N/2, instead of the old N/2.
- Panics when an Ord implementation violates strict weak ordering, instead of returning arbitrary order.

## Problems with the evidence

- Numbers are from the authors' own suite, measured on a 2024 nightly before the merge.
- rustc-perf showed compile-time instruction-count regressions on the PR, mostly from extra code generated per sort instantiation.

## Evidence

<details>
<summary>6 quotes from the sources, checked 2026-10-28</summary>

> random shows a fairly flat ~2.4x throughput improvement for N > 1e4 and <= 1e6.
>
> — [github.com/Voultapher/sort-research-rs/blob/main/writeup/driftsort_…](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/driftsort_introduction/text.md)

> random_d20 and random_p5 peak at 1e6, with a respective improvement of ~13x and ~17x.
>
> — [github.com/Voultapher/sort-research-rs/blob/main/writeup/driftsort_…](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/driftsort_introduction/text.md)

> release                      | `u64`    | 2335                 | 25520                  | 5294
>
> — [github.com/Voultapher/sort-research-rs/blob/main/writeup/driftsort_…](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/driftsort_introduction/text.md)

> rustc 1.77.0-nightly (6ae4cfbbb 2024-01-17)
>
> — [github.com/Voultapher/sort-research-rs/blob/main/writeup/driftsort_…](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/driftsort_introduction/text.md)

> AMD Ryzen 9 5900X 12-Core Processor (Zen 3 micro-architecture)
>
> — [github.com/Voultapher/sort-research-rs/blob/main/writeup/driftsort_…](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/driftsort_introduction/text.md)

> Both the stable and unstable sort implementations in the standard library have been updated to new algorithms, improving their runtime performance and compilation time.
>
> — [blog.rust-lang.org/2024/09/05/Rust-1.81.0/](https://blog.rust-lang.org/2024/09/05/Rust-1.81.0/)

</details>
