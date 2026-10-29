# SIMD

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Numerics and media](../README.md)</sub>

**The default:** Scalar loops left to LLVM's auto-vectorizer.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**wide (f32x4/f32x8 SIMD types, measured through ultraviolet and nalgebra/simba AoSoA types)**](lokathor-wide.md) | Stable-Rust SIMD vector types; AoSoA math built on them runs 4-12x faster than one-at-a-time glam in mathbench, with target-cpu=native. | A | 🟡 Holds up in part |
| [**std::simd (portable SIMD, nightly only)**](std-simd-portable-simd.md) | Portable SIMD vector types in std; can beat auto-vectorized loops, but still nightly-only and with little published Rust benchmark data. | C | 🟡 Holds up in part |
| [**arrow-rs `simd` feature (hand-written packed_simd arithmetic kernels)**](apache-arrow-rs-simd-feature.md) | Arrow's hand-written packed_simd kernels lost to auto-vectorized scalar loops (1.5-2.1x slower) and were removed; the default won. | A | ❌ Doesn't hold up |
