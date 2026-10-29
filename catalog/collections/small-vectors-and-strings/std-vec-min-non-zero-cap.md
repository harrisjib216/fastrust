# std Vec growth skips capacities 1 and 2 (Rust 1.45)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md) › [Small vectors and strings](README.md)</sub>

**Tier B** (independent measurement) · ✅ Holds up · Ships with Rust · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Since Rust 1.45, a Vec grown by push starts at capacity 4 (8 for bytes) instead of 1, removing two reallocations for small Vecs.

| | |
|---|---|
| **Beats** | Vec before Rust 1.45, whose push growth went 0, 1, 2, 4, 8 |
| **Claimed result** | Allocations done by rustc reduced by 10% or more; many rustc-perf benchmarks up to 4% faster. |
| **Workload** | The Rust compiler compiling the rustc-perf benchmark crates. |
| **Metric** | allocation count; compile time |
| **Hardware** | rust-lang perf server |
| **Toolchain** | rustc nightly, May 2020 |
| **Measured** | 2020-05 |
| **Platforms** | All targets |
| **Install** | built into std since 1.45 |
| **Links** | [documentation](https://doc.rust-lang.org/std/vec/struct.Vec.html) · [benchmark](https://github.com/rust-lang/rust/pull/72227) |

## Verdict

Measured on the compiler itself, starting pushed Vecs at capacity 4 (8 for 1-byte elements) cut rustc's allocations by 10% or more and sped up many rustc-perf benchmarks by up to 4%. Part of what small-vector crates were used for, avoiding the first few reallocations, is now the default.

**Why tier B:** The rust-lang project measuring a std change on its rustc-perf suite, reported in the PR and the author's follow-up post; no per-benchmark table in text.

## Use it when

- Always: it is the default Vec behavior. Before reaching for SmallVec or ArrayVec, check whether reallocation or the one remaining allocation is the cost.

## Skip it when

- You need zero heap allocations for small collections: a single allocation remains, which inline-storage crates avoid.

## How it is fast

The amortized growth path now jumps from 0 straight to a minimum non-zero capacity (8 for 1-byte elements, 4 for elements up to 1 KiB, 1 above that). Short Vecs built by push do one allocation instead of three, which matters in allocation-heavy code such as the compiler.

## What it costs

- Small Vecs may hold up to 3 unused slots; the PR author found no memory increase in practice.

## Problems with the evidence

- Numbers are aggregate figures from the compiler's own workloads; no microbenchmark of Vec itself was published.
- reserve_exact and with_capacity are unaffected and can still give capacity 1 or 2.

## Evidence

<details>
<summary>3 quotes from the sources, checked 2026-10-28</summary>

> This change reduces the number of allocations done by rustc itself by 10% or more.
>
> — [github.com/rust-lang/rust/pull/72227](https://github.com/rust-lang/rust/pull/72227)

> the capacity growth sequence is 0, 1, 2, 4, 8, 16, etc.
>
> — [github.com/rust-lang/rust/pull/72227](https://github.com/rust-lang/rust/pull/72227)

> sped up many benchmarks by up to 4%
>
> — [blog.mozilla.org/nnethercote/2020/08/05/how-to-speed-up-the-rust-co…](https://blog.mozilla.org/nnethercote/2020/08/05/how-to-speed-up-the-rust-compiler-some-more-in-2020/)

</details>
