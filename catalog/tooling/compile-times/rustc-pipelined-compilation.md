# Pipelined compilation in Cargo (Rust 1.38)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Build and tooling](../README.md) › [Compile times](README.md)</sub>

**Tier B** (independent measurement) · ✅ Holds up · Ships with Rust · `Rust` · 🟢 active

> Since Rust 1.38, Cargo starts compiling a crate as soon as its dependencies' metadata is ready, not after they finish.

| | |
|---|---|
| **Beats** | Cargo before Rust 1.38, which waited for each dependency to finish compiling before starting crates that depend on it |
| **Claimed result** | 10-20% faster optimized, clean builds of some crate graphs; no effect on single-crate builds |
| **Workload** | Clean release builds of multi-crate dependency graphs |
| **Metric** | build time |
| **Toolchain** | Rust 1.38.0 |
| **Measured** | 2019-09 |
| **Platforms** | All targets |
| **Install** | Built into Cargo since Rust 1.38; no configuration |
| **Links** | [documentation](https://blog.rust-lang.org/2019/09/26/Rust-1.38.0/) · [benchmark](https://blog.rust-lang.org/2019/09/26/Rust-1.38.0/) |

## Verdict

Rust 1.38 reported 10-20% faster optimized clean builds for some crate graphs during testing. It has been on by default since 2019, so every current toolchain already has it.

**Why tier B:** The rust-lang project reporting on its own toolchain change; the figure summarizes test reports rather than one published benchmark.

## Use it when

- Always on; nothing to enable

## Skip it when

- Single-crate builds see no change

## How it is fast

To type-check a crate, rustc needs only its dependencies' metadata (types, exports), which is produced early in compilation. Cargo now starts dependent crates as soon as that metadata exists, overlapping their front end with the dependency's code generation and using more cores.

## What it costs

- None for users

## Problems with the evidence

- The 10-20% range is from test reports in 2019; the post says other graphs did not improve much and results depend on hardware

## Evidence

<details>
<summary>2 quotes from the sources, checked 2026-10-28</summary>

> While the change doesn't have any effect on builds for a single crate, during testing we got reports of 10-20% compilation speed increases for optimized, clean builds of some crate graphs. Other ones did not improve much, and the speedup depends on the hardware running the build, so your mileage might vary. No code changes are needed to benefit from this.
>
> — [blog.rust-lang.org/2019/09/26/Rust-1.38.0/](https://blog.rust-lang.org/2019/09/26/Rust-1.38.0/)

> Starting with Rust 1.38.0, Cargo will take advantage of this by automatically starting to build dependent crates as soon as metadata is ready.
>
> — [blog.rust-lang.org/2019/09/26/Rust-1.38.0/](https://blog.rust-lang.org/2019/09/26/Rust-1.38.0/)

</details>
