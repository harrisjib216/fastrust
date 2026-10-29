# Linear algebra

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Numerics and media](../README.md)</sub>

**The default:** nalgebra and ndarray.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**glam (SIMD 3D math types)**](bitshifter-glam-rs.md) | 3D math library with SIMD-backed Vec4/Mat4/Quat; 2-12x faster than nalgebra on matrix and transform ops, slower on some vector ops. | A | 🟡 Holds up in part |
| [**faer (dense linear algebra)**](sarah-quinones-faer-rs.md) | Pure-Rust dense linear algebra; 5-13x faster than nalgebra on large decompositions, about even on matmul and slower on tiny matrices. | C | 🟡 Holds up in part |
