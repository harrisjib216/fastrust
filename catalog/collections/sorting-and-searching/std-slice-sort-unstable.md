# std slice::sort_unstable (ipnsort, Rust 1.81)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Sorting and searching](README.md)</sub>

**Tier A** (reproducible public benchmark) · ✅ Holds up · Ships with Rust · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Since Rust 1.81, slice::sort_unstable is ipnsort: a faster, smaller pdqsort successor with a sorting-network small-sort.

| | |
|---|---|
| **Beats** | slice::sort_unstable before Rust 1.81 (pdqsort) |
| **Claimed result** | Small-sort about 2-3x faster and partition about 1.2x for u64 on Zen 3; 1 KiB elements with 95% presorted input about 2.2x faster for N > 400 and < 2e4, rising to about 2.8x; binary size per u64 instantiation 4691 -> 3456 bytes. |
| **Workload** | Unstable sort of u64, i32, String, a 1 KiB struct and f128-like floats across patterns and lengths up to 1e7. |
| **Metric** | throughput |
| **Hardware** | AMD Ryzen 9 5900X (Zen 3); Intel i7-5500U; Apple M1 Pro |
| **OS** | Linux 6.6 (Zen 3); Linux 5.19; macOS (Darwin 22.6.0) |
| **Toolchain** | rustc 1.77.0-nightly (6ae4cfbbb 2024-01-17) |
| **Measured** | 2024-01 |
| **Platforms** | All targets |
| **Install** | built into std since 1.81 |
| **Links** | [documentation](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable) · [benchmark](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/ipnsort_introduction/text.md) |

## Verdict

ipnsort is faster than the old pdqsort across nearly all measured patterns and lengths on Zen 3, mainly through a 2x to 3x faster small-sort, and it is also smaller in binary size. A few partially sorted patterns between about 20 and 200 elements regress.

**Why tier A:** Public benchmark code (sort-research-rs) with CPU, OS and rustc version stated; the authors also wrote the std change.

## Use it when

- Default unstable sorting of any type.

## Skip it when

- Sorting large arrays of small integers or floats, where a radix sort can be faster; the authors say so themselves.
- Inputs of about 20 to 200 elements that are 95% presorted, which regressed.

## How it is fast

ipnsort keeps pdqsort's introsort structure but uses a branchless Lomuto-style cyclic partition and sorting networks for small slices, which avoid branch mispredictions. It inlines only the tiny-input insertion sort and outlines the main loop, reducing instruction-cache misses.

## What it costs

- Panics when an Ord implementation violates strict weak ordering.
- Small regressions for nearly sorted short inputs.

## Problems with the evidence

- Authors' own measurements on a pre-merge nightly; detailed discussion covers Zen 3 only.
- The authors note some ascending/descending gains do not reproduce on Apple M1 or older rustc.

## Evidence

<details>
<summary>5 quotes from the sources, checked 2026-10-28</summary>

> the new partition implementation contributes a ~1.2x overall improvement for `u64` on Zen 3. While the small-sort is ~2-3x times faster.
>
> — [github.com/Voultapher/sort-research-rs/blob/main/writeup/ipnsort_in…](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/ipnsort_introduction/text.md)

> random_s95 see a large improvement for N > 400 and < 2e4, with a roughly flat ~2.2x improvement. For N > 2e4 the throughput keeps rising to ~2.8x.
>
> — [github.com/Voultapher/sort-research-rs/blob/main/writeup/ipnsort_in…](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/ipnsort_introduction/text.md)

> release                      | `u64`    | 4691                 | 3456
>
> — [github.com/Voultapher/sort-research-rs/blob/main/writeup/ipnsort_in…](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/ipnsort_introduction/text.md)

> For very small types `mem::size_of::<T>() <= mem::size_of::<16>()` a good counting or radix sort is almost certainly faster.
>
> — [github.com/Voultapher/sort-research-rs/blob/main/writeup/ipnsort_in…](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/ipnsort_introduction/text.md)

> Both the stable and unstable sort implementations in the standard library have been updated to new algorithms, improving their runtime performance and compilation time.
>
> — [blog.rust-lang.org/2024/09/05/Rust-1.81.0/](https://blog.rust-lang.org/2024/09/05/Rust-1.81.0/)

</details>
