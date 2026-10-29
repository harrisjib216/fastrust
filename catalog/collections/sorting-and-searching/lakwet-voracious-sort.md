# voracious_radix_sort (MSD/LSD radix sorts)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Sorting and searching](README.md)</sub>

**Tier C** (first-party numbers) · 🟡 Holds up in part · Open source · `Rust` · `MIT` · 🟢 active

> Radix sorts for Rust primitives and keyed structs; about 3x faster than pre-1.81 sort_unstable on a million u64.

| | |
|---|---|
| **Beats** | slice::sort_unstable (pdqsort) and slice::sort before Rust 1.81 |
| **Claimed result** | u64, 1,000,000 uniform values: 11.94 ns per item vs 37.39 for sort_unstable and 58.89 for sort (about 3.1x and 4.9x, derived from page data); 100 values: 27.12 vs 26.25 ns per item (no gain). |
| **Workload** | Single-threaded sort of u64 arrays from 100 to 10,000,000 elements across uniform, small-range, sorted and other distributions, 5 iterations each. |
| **Metric** | time (ns per item) |
| **Hardware** | AMD Ryzen 9 3950X 16c/32t |
| **Measured** | 2023-03 |
| **Adoption** | 78 stars |
| **Last commit** | 2025-12-08 |
| **Platforms** | All Rust targets; multithreaded variants use rayon |
| **Install** | cargo add voracious_radix_sort |
| **Links** | [repository](https://github.com/lakwet/voracious_sort) · [documentation](https://docs.rs/voracious_radix_sort) · [benchmark](https://github.com/lakwet/voracious_sort/blob/master/results/benchmark_1_2_0/benchmark_results_trait_u64) |

## Verdict

On large integer arrays the radix sorts beat the std sorts of their time by about 3x (unstable) to 5x (stable) on u64. The baseline was pre-1.81 std; ipnsort and driftsort narrow the gap, though the ipnsort authors still expect radix sorts to win on small integer types. Below about 1,000 elements there is no gain.

**Why tier C:** First-party results files with the CPU named, but no rustc version or flags for the run and no independent reproduction.

## Use it when

- Sorting large arrays (roughly 10^4 elements or more) of integers, floats or structs with an integer key.

## Skip it when

- Short arrays: it falls back to std sorts, and below about 1,000 elements the results show no gain.
- Comparison-defined orders or strings, which it does not support.
- Memory is tight: some variants use up to 2x the array size.

## How it is fast

Radix sorts place elements by the bits of their key instead of comparing them, doing a fixed number of linear passes per byte of key. The crate picks among MSD, LSD and hybrid variants by type and length and falls back to std sorts for small inputs.

## What it costs

- Out-of-place variants allocate up to the array size again.
- Sorting custom types requires implementing the crate's Radixable trait.

## Problems with the evidence

- The baseline std sorts in these results predate Rust 1.81's ipnsort and driftsort; no newer comparison was found.
- Tuning constants were profiled on the author's Ryzen 3950X and may not fit other CPUs.
- No rustc version or optimization flags are stated for the 1.2.0 results; the README's last tested toolchain is Rust 1.46.

## Evidence

<details>
<summary>7 quotes from the sources, checked 2026-10-28</summary>

> Array size: 1_000_000
>
> — [github.com/lakwet/voracious_sort/blob/master/results/benchmark_1_2_…](https://github.com/lakwet/voracious_sort/blob/master/results/benchmark_1_2_0/benchmark_results_trait_u64)

> -- Unif       :11935us	5357ns	(11.94ns)	5938us	2690ns	(5.94ns)	37385us	17148ns	(37.39ns)	58894us	26339ns	(58.89ns)	6876us	3337ns	(6.88ns)
>
> — [github.com/lakwet/voracious_sort/blob/master/results/benchmark_1_2_…](https://github.com/lakwet/voracious_sort/blob/master/results/benchmark_1_2_0/benchmark_results_trait_u64)

> -- Unif       :2us	221ns	(27.12ns)	2us	118ns	(22.94ns)	2us	130ns	(26.25ns)	2us	132ns	(27.25ns)	2us	111ns	(23.74ns)
>
> — [github.com/lakwet/voracious_sort/blob/master/results/benchmark_1_2_…](https://github.com/lakwet/voracious_sort/blob/master/results/benchmark_1_2_0/benchmark_results_trait_u64)

> === Test u64 ===	Trait Vora Uns or stable		Trait Vora MT		Rust Uns		Rust Std		Rayon pll uns
>
> — [github.com/lakwet/voracious_sort/blob/master/results/benchmark_1_2_…](https://github.com/lakwet/voracious_sort/blob/master/results/benchmark_1_2_0/benchmark_results_trait_u64)

> CPU: AMD Ryzen 3950x 16c/32t
>
> — [github.com/lakwet/voracious_sort/blob/master/results/benchmark_1_2_…](https://github.com/lakwet/voracious_sort/blob/master/results/benchmark_1_2_0/computer_spec)

> It is way faster than Rust standard sort and Rust unstable sort
>
> — [github.com/lakwet/voracious_sort/blob/master/README.md](https://github.com/lakwet/voracious_sort/blob/master/README.md)

> Array size: 100
>
> — [github.com/lakwet/voracious_sort/blob/master/results/benchmark_1_2_…](https://github.com/lakwet/voracious_sort/blob/master/results/benchmark_1_2_0/benchmark_results_trait_u64)

</details>
