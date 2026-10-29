# once_cell (sync::Lazy)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md) › [Lazy statics and one-time init](README.md)</sub>

**Tier B** (independent measurement) · ❌ Doesn't hold up · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> Lazy statics without a macro, now mostly in std as OnceLock and LazyLock; access speed matches lazy_static, so pick it for API, not speed.

| | |
|---|---|
| **Beats** | lazy_static 1.4 (lazy_static! macro) |
| **Claimed result** | Claim: once_cell 'is about 20% faster in a [32-thread test]' than lazy_static. Measured on the same page: once_cell_lazy 26 ns/iter vs lazy_static_local 27 ns/iter, and a plain local regex at 27 ns/iter. |
| **Workload** | Repeated regex is_match on an email string, with the Regex compiled once and stored in a lazy_static!, a once_cell::sync::Lazy, or a local variable; access cost after initialization. |
| **Metric** | time |
| **Measured** | 2022 |
| **Adoption** | 2.1k stars |
| **Last commit** | 2026-03-12 |
| **Platforms** | All targets; no_std via the race module or critical-section feature |
| **Install** | cargo add once_cell |
| **Links** | [repository](https://github.com/matklad/once_cell) · [documentation](https://docs.rs/once_cell) · [benchmark](https://github.com/rimutaka/empirical) |

## Verdict

The only speed claim found, 'about 20% faster in a 32-thread test', links to a once_cell example with no published results that is no longer in the repository. The same page's own benchmark measured 26 vs 27 ns per iteration, within noise. No measured speed difference separates lazy_static, once_cell and std's OnceLock (1.70) and LazyLock (1.80).

**Why tier B:** A third-party benchmark independent of both crates' authors, with code in the repository but no hardware or toolchain stated.

## Use it when

- MSRV below 1.70 (OnceLock) or 1.80 (LazyLock)
- no_std code that needs the race module
- Features std lacks, such as get_or_try_init on stable

## Skip it when

- Rust 1.80 or newer with no extra features needed: std::sync::LazyLock does the same job with no dependency
- Expecting a speedup over lazy_static: none was measured

## How it is fast

After initialization, reading a Lazy is one acquire load of a state word followed by a pointer dereference, the same fast path lazy_static and std's LazyLock use. Initialization goes through a Once-style slow path that runs exactly once.

## What it costs

- A dependency for functionality std now provides.
- The crate uses unsafe internally, as do std's equivalents.

## Problems with the evidence

- The benchmarked operation is dominated by the regex match, so lazy-access overhead differences would be hidden.
- The 20% figure has no published numbers behind it; the linked example file returns 404 on the current master branch.
- No hardware or toolchain stated; cargo bench output with single-digit ns/iter noise.

## Evidence

<details>
<summary>6 quotes from the sources, checked 2026-10-28</summary>

> and is about 20% faster in a [32-thread test](https://github.com/matklad/once_cell/blob/master/examples/bench_vs_lazy_static.rs). It performed on the par with `lazy_static!` in my basic benchmark.
>
> — [github.com/rimutaka/empirical](https://github.com/rimutaka/empirical)

> test lazy_static_local ... bench: 27 ns/iter (+/- 5)
>
> — [github.com/rimutaka/empirical](https://github.com/rimutaka/empirical)

> test once_cell_lazy ... bench: 26 ns/iter (+/- 2)
>
> — [github.com/rimutaka/empirical](https://github.com/rimutaka/empirical)

> test vanilla_rust_local ... bench: 27 ns/iter (+/- 0)
>
> — [github.com/rimutaka/empirical](https://github.com/rimutaka/empirical)

> If you can use `std` version (your MSRV is at least 1.70, and you don't need
>
> — [github.com/matklad/once_cell/blob/master/src/lib.rs](https://github.com/matklad/once_cell/blob/master/src/lib.rs)

> Don't use `lazy_static`.
>
> — [github.com/matklad/once_cell/blob/master/src/lib.rs](https://github.com/matklad/once_cell/blob/master/src/lib.rs)

</details>
