# wide (f32x4/f32x8 SIMD types, measured through ultraviolet and nalgebra/simba AoSoA types)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Numerics and media](../README.md) › [SIMD](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `Zlib OR Apache-2.0 OR MIT` · 🟢 active

> Stable-Rust SIMD vector types; AoSoA math built on them runs 4-12x faster than one-at-a-time glam in mathbench, with target-cpu=native.

| | |
|---|---|
| **Beats** | Scalar per-element math with glam 0.34.1 (one Vec3/Mat4 at a time) doing the same total work |
| **Claimed result** | ns per element, glam_f32x1 vs ultraviolet_f32x8: ray-sphere intersection 9.2936 vs 0.7473, vector3 length 1.2698 vs 0.1621, matrix4 inverse 11.0112 vs 2.8535, matrix4 mul matrix4 3.1637 vs 2.5313 (12.4x, 7.8x, 3.9x, 1.25x) (derived from page data). ultraviolet_f32x4: ray-sphere 1.2298, vector3 length 0.3208 ns (7.6x, 4.0x) (derived from page data). Slower: matrix4 transpose 1.4185 vs 2.4720 ns |
| **Workload** | mathbench "wide" benchmarks: AoSoA (array of structs of arrays) versions of vector, matrix, rotation and ray-sphere operations over 16 to 80,000 elements, compared with scalar glam doing the same total work; time per element. |
| **Metric** | time |
| **Hardware** | AMD Ryzen 7 3800X |
| **OS** | Linux |
| **Toolchain** | rustc 1.99.0 (b940084d7 2026-09-28), default bench profile plus RUSTFLAGS='-C target-cpu=native' |
| **Measured** | 2026-10-13 |
| **Adoption** | 558 stars |
| **Last commit** | 2026-10-28 |
| **Platforms** | x86/x86_64 (SSE2 baseline, AVX/AVX2 when enabled at build time), aarch64 NEON, wasm32 simd128, scalar fallback elsewhere; stable Rust |
| **Install** | cargo add wide |
| **Links** | [repository](https://github.com/Lokathor/wide) · [documentation](https://docs.rs/wide) · [benchmark](https://github.com/bitshifter/mathbench-rs) |

## Verdict

Rewriting math in structure-of-arrays form on wide-backed f32x8 types gives 3.9x to 12.4x more throughput than one-at-a-time glam, but only after restructuring data and algorithms, and all published numbers use target-cpu=native (f32x8 needs AVX2). Some operations, such as 4x4 transpose, get slower.

**Why tier A:** mathbench is public, and the README states the rustc version, RUSTFLAGS, CPU and library versions.

## Use it when

- You process many independent vectors or rays (particles, physics, ray tracing) and can lay data out as structure-of-arrays.
- You need explicit SIMD on stable Rust without writing per-architecture intrinsics.

## Skip it when

- Your algorithm works on one vector at a time or needs horizontal operations; transposes measured slower.
- You ship generic x86_64 binaries: f32x8 falls back to two 128-bit halves without AVX2 enabled at build time, and wide does no runtime dispatch.

## How it is fast

wide provides f32x4, f32x8 and integer lane types that map to SSE, AVX or NEON registers selected at compile time. Libraries such as ultraviolet and simba store 4 or 8 vectors per struct, one lane per vector, so each instruction processes 4 or 8 independent elements instead of one.

## What it costs

- Requires restructuring data into AoSoA form and rewriting branches as lane masks.
- Instruction set is fixed at compile time; no runtime feature detection.

## Problems with the evidence

- All numbers use target-cpu=native on a Ryzen 7 3800X; mathbench notes that the f32x8 benchmarks require AVX2.
- The benchmark measures wide through ultraviolet 0.10.0 (which depends on wide 0.7) and nalgebra's simba types, not wide directly, and against a baseline that already uses SSE2 per vector.
- mathbench itself warns that scalar and wide results are not a direct comparison because the algorithms are restructured.

## Evidence

<details>
<summary>10 quotes from the sources, checked 2026-10-28</summary>

> | ray-sphere intersection x80000 |    9.2936 ns   |           1.2298 ns   |        1.2658 ns   |         __0.7473 ns__ |        0.7690 ns   |
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> | vector3 length x1024           |    1.2698 ns   |           0.3208 ns   |        0.3200 ns   |         __0.1621 ns__ |      __0.1615 ns__ |
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> | matrix4 inverse x1024          |   11.0112 ns   |           4.4601 ns   |          N/A       |         __2.8535 ns__ |          N/A       |
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> | matrix4 mul matrix4 x1024      |    3.1637 ns   |           3.4517 ns   |        7.4823 ns   |         __2.5313 ns__ |        4.5643 ns   |
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> | matrix4 transpose x1024        |  __1.4185 ns__ |           5.6545 ns   |        3.7571 ns   |           2.4720 ns   |        1.9211 ns   |
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> RUSTFLAGS='-C target-cpu=native' cargo bench --features wide wide
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> Note that the `nalgebra` wide benchmarks use `simba`'s stable, `wide` crate backed types (`WideF32x4` and `WideF32x8`), so they build on stable Rust.
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> Additionally the `f32x8` benchmarks will require the `AVX2` instruction set
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> compiled with the `1.99.0 (b940084d7 2026-09-28)` Rust compiler with
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> * `ultraviolet` - `0.10.0`
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

</details>
