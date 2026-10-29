# panic = "abort" in the release profile

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md) › [Build settings for faster binaries](README.md)</sub>

**Tier D** (directional only) · 🟡 Holds up in part · Ships with Rust · `Rust` · 🟢 active

> Aborting instead of unwinding on panic shrinks binaries; any runtime gain is described as slight and is unmeasured.

| | |
|---|---|
| **Beats** | cargo build --release with the default panic = "unwind" |
| **Claimed result** | No numbers: 'might reduce binary size and increase runtime speed slightly' (Rust Performance Book); 'another small reduction in warm build time' (third-party warm-build test) |
| **Workload** | None published: release builds with panic = "abort" against unwinding |
| **Metric** | time; binary size |
| **Platforms** | All targets |
| **Install** | [profile.release] panic = "abort" |
| **Links** | [documentation](https://doc.rust-lang.org/cargo/reference/profiles.html) · [benchmark](https://nnethercote.github.io/perf-book/build-configuration.html) |

## Verdict

The Rust Performance Book only says it might increase runtime speed slightly, and no published benchmark gives a number. The measurable effects reported are smaller binaries and slightly faster builds, and it changes program behaviour.

**Why tier D:** Directional statements only; no runtime numbers in any source found.

## Use it when

- Binaries that never catch panics and want smaller output

## Skip it when

- Code that uses catch_unwind, or servers that isolate panics per request or thread
- Libraries whose users may rely on unwinding
- Tests, benchmarks, build scripts and proc macros, which ignore the setting anyway

## How it is fast

With unwinding, the compiler emits landing pads and unwind tables so destructors run while a panic propagates. With abort, a panic ends the process at once, so that code is not generated; the main reported effect is a smaller binary.

## What it costs

- Destructors do not run on panic
- No recovery with catch_unwind

## Problems with the evidence

- No published runtime benchmark found; treat any speed gain as unproven for your program

## Evidence

<details>
<summary>3 quotes from the sources, checked 2026-10-28</summary>

> This might reduce binary size and increase runtime speed slightly, and may even reduce compile times slightly.
>
> — [nnethercote.github.io/perf-book/build-configuration.html](https://nnethercote.github.io/perf-book/build-configuration.html)

> which from my measurements gives another small reduction in warm build time.
>
> — [davidlattimore.github.io/posts/2024/02/04/speeding-up-the-rust-edit…](https://davidlattimore.github.io/posts/2024/02/04/speeding-up-the-rust-edit-build-run-cycle.html)

> Tests, benchmarks, build scripts, and proc macros ignore the panic setting.
>
> — [doc.rust-lang.org/cargo/reference/profiles.html](https://doc.rust-lang.org/cargo/reference/profiles.html)

</details>
