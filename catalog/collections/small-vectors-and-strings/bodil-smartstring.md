# smartstring (SmartString inline string)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Small vectors and strings](README.md)</sub>

**Tier D** (directional only) · ❌ Doesn't hold up · Open source · `Rust` · `MPL-2.0` · 🔴 abandoned

> Archived inline-string String replacement; it saves allocations, but an independent benchmark drops it from access tests for being slow.

| | |
|---|---|
| **Beats** | String for short strings |
| **Claimed result** | No speed numbers in text. Allocation events 19 vs 2017 for String, peak memory 73.8 KB vs 82.7 KB, loading 1000 records (debug build). |
| **Workload** | Deserializing 1,000 JSON city records (allocation counts); string-rosetta-rs create, clone, access and compare benchmarks (charts). |
| **Metric** | memory (allocation count); time (charts only) |
| **Toolchain** | debug build for the allocation counts (smartstring 0.2.2, 2020); string-rosetta-rs run of 2026-03-09 for the access result |
| **Measured** | 2026-03-09 |
| **Adoption** | 543 stars |
| **Last commit** | 2022-03-24 |
| **Platforms** | All Rust targets (23-byte inline limit on 64-bit) |
| **Install** | cargo add smartstring |
| **Links** | [repository](https://github.com/bodil/smartstring) · [documentation](https://docs.rs/smartstring) · [benchmark](https://github.com/rosetta-rs/string-rosetta-rs) |

## Verdict

The allocation savings are real (19 allocation events against 2,017 for String in an independent 2020 test), but the README's claim that inlining makes strings faster overall is contradicted by the current independent string benchmark, which leaves smartstring out of its access test because it is too slow. The repository is archived.

**Why tier D:** The speed evidence is a third-party remark with charts only; the only text numbers are allocation counts from a 2020 debug build.

## Use it when

- Existing code already depends on it and allocation count, not access speed, is the measured cost.

## Skip it when

- New code: compact_str offers the same inline-string idea, is maintained, and was not singled out as slow.
- Hot paths that read string contents often: string-rosetta-rs found access too slow to chart.

## How it is fast

SmartString stores strings up to 23 bytes inside the 24 bytes a String occupies, using pointer alignment to tell inline and heap layouts apart, so short strings need no allocation. Longer strings are a regular String, and converting between the two reuses the buffer.

## What it costs

- Every access must check which layout is in use, which adds a branch over String.
- Archived repository: no fixes or updates.

## Problems with the evidence

- The access result is a remark next to a chart, with no published numbers.
- Allocation counts come from a 2020 article run in a debug build with smartstring 0.2.2; the current release is 1.0.1 (2022).

## Evidence

<details>
<summary>6 quotes from the sources, checked 2026-10-28</summary>

> making strings go faster overall.
>
> — [github.com/bodil/smartstring/blob/master/README.md](https://github.com/bodil/smartstring/blob/master/README.md)

> *(`smartstring` is skipped due to how slow it is)*
>
> — [github.com/rosetta-rs/string-rosetta-rs/blob/master/README.md](https://github.com/rosetta-rs/string-rosetta-rs/blob/master/README.md)

> down to 19 allocation events, and peak memory usage of 73.8 KB.
>
> — [fasterthanli.me/articles/small-strings-in-rust](https://fasterthanli.me/articles/small-strings-in-rust)

> alloc events | 2017
>
> — [fasterthanli.me/articles/small-strings-in-rust](https://fasterthanli.me/articles/small-strings-in-rust)

> peak bytes | 82.7 KB
>
> — [fasterthanli.me/articles/small-strings-in-rust](https://fasterthanli.me/articles/small-strings-in-rust)

> Read 1000 records
>
> — [fasterthanli.me/articles/small-strings-in-rust](https://fasterthanli.me/articles/small-strings-in-rust)

</details>
