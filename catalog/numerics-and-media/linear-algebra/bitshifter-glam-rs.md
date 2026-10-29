# glam (SIMD 3D math types)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Numerics and media](../README.md) › [Linear algebra](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `MIT OR Apache-2.0` · 🟢 active

> 3D math library with SIMD-backed Vec4/Mat4/Quat; 2-12x faster than nalgebra on matrix and transform ops, slower on some vector ops.

| | |
|---|---|
| **Beats** | nalgebra 0.35.0 small fixed-size f32 vectors, matrices and transforms |
| **Claimed result** | ns per op, glam vs nalgebra: matrix4 mul matrix4 3.2445 vs 6.3150, matrix4 inverse 10.9287 vs 26.7860, matrix4 determinant 3.4266 vs 40.7346, transform3 inverse 5.8707 vs 26.5732, ray-sphere intersection 8.0682 vs 17.5249 (1.95x, 2.45x, 11.9x, 4.53x, 2.17x) (derived from page data). Slower: vector3 length 1.2799 vs 0.3107, vector3 normalize 2.1077 vs 0.9517 ns |
| **Workload** | mathbench scalar benchmarks on f32: one operation applied over batches of 16 or 1,024 inputs (10,000 for ray-sphere and Euler), criterion timings per operation. |
| **Metric** | time |
| **Hardware** | AMD Ryzen 7 3800X |
| **OS** | Linux |
| **Toolchain** | rustc 1.99.0 (b940084d7 2026-09-28), default bench profile plus RUSTFLAGS='-C target-cpu=native' |
| **Measured** | 2026-10-13 |
| **Adoption** | 2.1k stars |
| **Last commit** | 2026-10-18 |
| **Platforms** | SIMD storage on x86, x86_64, aarch64 and wasm32/wasm64 (SSE2 by default on x86_64, NEON on aarch64, simd128 opt-in); scalar elsewhere |
| **Install** | cargo add glam |
| **Links** | [repository](https://github.com/bitshifter/glam-rs) · [documentation](https://docs.rs/glam) · [benchmark](https://github.com/bitshifter/mathbench-rs) |

## Verdict

glam is 2x to 12x faster than nalgebra on 4x4 matrix multiply, inverse and determinant, affine inverse and ray-sphere tests, but in the same run it is 4.1x slower on vector3 length and 2.2x slower on vector3 normalize. The win is real for matrix and transform work, not across the board.

**Why tier A:** mathbench is public, and the README states the rustc version, RUSTFLAGS, CPU and every library version; it is written by glam's author.

## Use it when

- Games, graphics and simulation code built on 2D-4D f32 or f64 vectors, matrices, quaternions and affine transforms.
- Hot loops over 4x4 matrix multiply, inverse or transform inverse.

## Skip it when

- You need general N-dimensional or dynamically sized linear algebra; glam stops at 4x4.
- Your hot path is vector3 length or normalize, where nalgebra measured faster in this run.

## How it is fast

Vec3A, Vec4, Mat4 and Quat are stored in 16-byte SIMD registers and implemented with SSE2 or NEON intrinsics, so a 4x4 multiply or inverse runs as a handful of vector instructions. nalgebra's generic code relies on the auto-vectorizer for fixed-size types, which handles matrix-matrix work less well.

## What it costs

- Fixed set of types up to 4x4; no generic dimensions or scalar types beyond f32, f64 and integer vectors.
- Vec3A and Mat3A are 16-byte aligned and padded, so they use more memory than packed [f32; 3].

## Problems with the evidence

- All published numbers use target-cpu=native on a Ryzen 7 3800X; default x86_64 builds get only SSE2.
- mathbench is written by glam's author.
- Microbenchmarks of single operations over small batches; whole-program gains depend on the mix of operations.

## Evidence

<details>
<summary>12 quotes from the sources, checked 2026-10-28</summary>

> | matrix4 mul matrix4 x1024        |  __3.2445 ns__ |    6.2365 ns   |    6.3150 ns   |    6.1518 ns   |    4.9908 ns   |      N/A       |     6.6484 ns   |
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> | matrix4 inverse x1024            |   10.9287 ns   |   25.2419 ns   |   26.7860 ns   |   31.8244 ns   |   22.2220 ns   |      N/A       |   __6.1286 ns__ |
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> | matrix4 determinant x1024        |    3.4266 ns   |  __2.3661 ns__ |   40.7346 ns   |    9.2307 ns   |   10.2196 ns   |      N/A       |   __2.3333 ns__ |
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> | transform3 inverse x1024         |  __5.8707 ns__ |      N/A       |   26.5732 ns   |   36.8259 ns   |      N/A       |   15.3330 ns   |       N/A       |
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> | ray-sphere intersection x10000   |    8.0682 ns   |   16.3118 ns   |   17.5249 ns   |   16.3678 ns   |   17.4790 ns   |      N/A       |   __5.7654 ns__ |
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> | vector3 length x1024             |    1.2799 ns   |  __0.3100 ns__ |  __0.3107 ns__ |  __0.3133 ns__ |  __0.3096 ns__ |      N/A       |   __0.3135 ns__ |
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> | vector3 normalize x1024          |    2.1077 ns   |    1.0334 ns   |  __0.9517 ns__ |  __0.9462 ns__ |  __0.9626 ns__ |      N/A       |     1.0223 ns   |
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> compiled with the `1.99.0 (b940084d7 2026-09-28)` Rust compiler with
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> RUSTFLAGS='-C target-cpu=native' cargo bench --features scalar scalar
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> These benchmarks were performed on an [AMD Ryzen 7 3800X] CPU on Linux.
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> * `nalgebra` - `0.35.0`
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

> * `glam` - `0.34.1`
>
> — [github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acb…](https://github.com/bitshifter/mathbench-rs/blob/5d2fb0650538f3668837a059acbd3a10d4cf6069/README.md)

</details>
