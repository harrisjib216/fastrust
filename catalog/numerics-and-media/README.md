# Numerics and media

<sub>[fastrust](../../README.md) › [Catalog](../README.md)</sub>

## [Linear algebra](linear-algebra/README.md)

**The default:** nalgebra and ndarray.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**glam (SIMD 3D math types)**](linear-algebra/bitshifter-glam-rs.md) | 3D math library with SIMD-backed Vec4/Mat4/Quat; 2-12x faster than nalgebra on matrix and transform ops, slower on some vector ops. | A | 🟡 Holds up in part |
| [**faer (dense linear algebra)**](linear-algebra/sarah-quinones-faer-rs.md) | Pure-Rust dense linear algebra; 5-13x faster than nalgebra on large decompositions, about even on matmul and slower on tiny matrices. | C | 🟡 Holds up in part |

## [SIMD](simd/README.md)

**The default:** Scalar loops left to LLVM's auto-vectorizer.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**wide (f32x4/f32x8 SIMD types, measured through ultraviolet and nalgebra/simba AoSoA types)**](simd/lokathor-wide.md) | Stable-Rust SIMD vector types; AoSoA math built on them runs 4-12x faster than one-at-a-time glam in mathbench, with target-cpu=native. | A | 🟡 Holds up in part |
| [**std::simd (portable SIMD, nightly only)**](simd/std-simd-portable-simd.md) | Portable SIMD vector types in std; can beat auto-vectorized loops, but still nightly-only and with little published Rust benchmark data. | C | 🟡 Holds up in part |
| [**arrow-rs `simd` feature (hand-written packed_simd arithmetic kernels)**](simd/apache-arrow-rs-simd-feature.md) | Arrow's hand-written packed_simd kernels lost to auto-vectorized scalar loops (1.5-2.1x slower) and were removed; the default won. | A | ❌ Doesn't hold up |

## [Images](images/README.md)

**The default:** The image crate.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**fast_image_resize (SIMD image resizing)**](images/cykooz-fast-image-resize.md) | SIMD image resizer that downsizes 5-27x faster than the image crate's resize, depending on filter, pixel format and CPU. | A | ✅ Holds up |
| [**image 0.25: JPEG decoding switched to zune-jpeg**](images/image-rs-image-zune-jpeg-decoder.md) | Since 0.25.0 the image crate decodes JPEG with zune-jpeg instead of jpeg-decoder; upgrading gets the 1.5-2x faster decoder for free. | C | ✅ Holds up |
| [**zune-png (PNG decoder)**](images/etemesi254-zune-image-zune-png.md) | Pure-Rust PNG/APNG decoder; faster than the png crate on 8-bit and interlaced images, slower on 16-bit and palette images. | C | 🟡 Holds up in part |
| [**zune-jpeg (JPEG decoder)**](images/etemesi254-zune-image-zune-jpeg.md) | Pure-Rust JPEG decoder, 1.5-2x faster than jpeg-decoder per its author; it is now the image crate's default JPEG decoder. | C | ❌ Doesn't hold up |
