# Images

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Numerics and media](../README.md)</sub>

**The default:** The image crate.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**fast_image_resize (SIMD image resizing)**](cykooz-fast-image-resize.md) | SIMD image resizer that downsizes 5-27x faster than the image crate's resize, depending on filter, pixel format and CPU. | A | ✅ Holds up |
| [**image 0.25: JPEG decoding switched to zune-jpeg**](image-rs-image-zune-jpeg-decoder.md) | Since 0.25.0 the image crate decodes JPEG with zune-jpeg instead of jpeg-decoder; upgrading gets the 1.5-2x faster decoder for free. | C | ✅ Holds up |
| [**zune-png (PNG decoder)**](etemesi254-zune-image-zune-png.md) | Pure-Rust PNG/APNG decoder; faster than the png crate on 8-bit and interlaced images, slower on 16-bit and palette images. | C | 🟡 Holds up in part |
| [**zune-jpeg (JPEG decoder)**](etemesi254-zune-image-zune-jpeg.md) | Pure-Rust JPEG decoder, 1.5-2x faster than jpeg-decoder per its author; it is now the image crate's default JPEG decoder. | C | ❌ Doesn't hold up |
