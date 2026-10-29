# Less debug info in dev builds (debug = false or "line-tables-only")

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md) › [Compile times](README.md)</sub>

**Tier B** (independent measurement) · ✅ Holds up · Ships with Rust · `Rust` · 🟢 active

> Turning off or trimming debug info in Cargo's dev profile cuts incremental rebuild times by up to 30-40%.

| | |
|---|---|
| **Beats** | cargo build with the dev profile's default debug = true (full debug info) |
| **Claimed result** | Up to 30-40% faster compilation in some situations, especially incremental rebuilds (rustc-perf, with lld); warm cargo run 7.539 s with full debuginfo vs 1.611 s with debug=0 and 1.507 s with strip="debuginfo" (about 4.7x, derived from page data); cold build 107 s vs 77 s with debug=0; Rust Performance Book: up to 20-40% for dev builds |
| **Workload** | rustc-perf benchmark suite with debuginfo forced off; warm cargo run after a trivial edit of a crate with heavy dependencies, plus its cold build |
| **Metric** | build time |
| **Hardware** | Intel i7-10510U laptop, 42 GB RAM (third-party test) |
| **OS** | Pop!_OS Linux (third-party test) |
| **Toolchain** | lld linker (rustc-perf run); mold 2.3.3 linker (third-party test) |
| **Measured** | 2025-05 |
| **Platforms** | All targets |
| **Install** | [profile.dev] debug = false, or debug = "line-tables-only" to keep line numbers in backtraces |
| **Links** | [documentation](https://doc.rust-lang.org/cargo/reference/profiles.html) · [benchmark](https://kobzol.github.io/rust/rustc/2025/05/20/disable-debuginfo-to-improve-rust-compile-times.html) |

## Verdict

A rustc-perf run by a compiler team member found builds up to 30-40% faster, mostly on incremental rebuilds, and an independent warm-rebuild test went from 7.5 s to 1.6 s. Cargo's dev profile still defaults to full debug info in Rust 1.99, which added a separate debug profile as the first step toward changing that.

**Why tier B:** Measured by a Rust compiler team member on the rust-lang benchmark suite and separately by a third party with a published setup; neither publishes a full results table in the text.

## Use it when

- You rarely run a debugger and debug with prints or logs
- Incremental rebuild time matters most

## Skip it when

- You step through code in a debugger: keep full debug info
- You need line numbers in backtraces: use line-tables-only rather than false

## How it is fast

Full DWARF debug info costs time during code generation, and the linker then copies and relocates large debug sections. Turning it off removes both costs; line-tables-only keeps file and line numbers for backtraces and most of the saving. Incremental rebuilds gain the most because linking is a larger share of their time.

## What it costs

- No variable or type information for debuggers
- With debug = false, backtraces lose line numbers
- On Linux, split-debuginfo = "unpacked" is a middle ground; it is already the default on macOS (since Rust 1.53)

## Problems with the evidence

- The 30-40% figure is the top of a range from one benchmark run, presented as a linked results page
- The third-party test is one crate on one laptop (February 2024), with mold already in place
- The standard library ships with debug info, so debug = 0 alone still links some of it

## Evidence

<details>
<summary>8 quotes from the sources, checked 2026-10-28</summary>

> In some situations, especially for incremental rebuilds, disabling debuginfo can make your compilation 30-40% faster!
>
> — [kobzol.github.io/rust/rustc/2025/05/20/disable-debuginfo-to-improve…](https://kobzol.github.io/rust/rustc/2025/05/20/disable-debuginfo-to-improve-rust-compile-times.html)

> That reduces our cargo run time to 7.539 s ± 1.691 s.
>
> — [davidlattimore.github.io/posts/2024/02/04/speeding-up-the-rust-edit…](https://davidlattimore.github.io/posts/2024/02/04/speeding-up-the-rust-edit-build-run-cycle.html)

> With just strip="debuginfo" our time comes down to 1.507 s ± 0.301 s.
>
> — [davidlattimore.github.io/posts/2024/02/04/speeding-up-the-rust-edit…](https://davidlattimore.github.io/posts/2024/02/04/speeding-up-the-rust-edit-build-run-cycle.html)

> With just debug=0 we’re slightly slower at 1.611 s ± 0.329 s
>
> — [davidlattimore.github.io/posts/2024/02/04/speeding-up-the-rust-edit…](https://davidlattimore.github.io/posts/2024/02/04/speeding-up-the-rust-edit-build-run-cycle.html)

> setting debug=0 also improves the cold build times (77 seconds down from 107)
>
> — [davidlattimore.github.io/posts/2024/02/04/speeding-up-the-rust-edit…](https://davidlattimore.github.io/posts/2024/02/04/speeding-up-the-rust-edit-build-run-cycle.html)

> consider disabling debuginfo. This can improve dev build times significantly, by as much as 20-40%.
>
> — [nnethercote.github.io/perf-book/build-configuration.html](https://nnethercote.github.io/perf-book/build-configuration.html)

> This is a preparation for transitioning the `dev` profile away from debugging to give a saner default for faster development iterations.
>
> — [github.com/rust-lang/rust/blob/main/RELEASES.md](https://github.com/rust-lang/rust/blob/main/RELEASES.md)

> macOS targets now default to `unpacked` split-debuginfo.
>
> — [github.com/rust-lang/rust/blob/main/RELEASES.md](https://github.com/rust-lang/rust/blob/main/RELEASES.md)

</details>
