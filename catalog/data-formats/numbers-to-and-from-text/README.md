# Numbers to and from text

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md)</sub>

**The default:** std's Display/format! and str::parse.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**ryu (Ryū float-to-string)**](dtolnay-ryu.md) | Ryū shortest float-to-string: 2.2x-2.4x faster than std's float Display, now overtaken by zmij. | A | ✅ Holds up |
| [**std float parsing (Eisel-Lemire, Rust 1.55)**](std-num-f64-from-str.md) | Since Rust 1.55, str::parse::<f64> uses Eisel-Lemire and runs within about 1.2x of fast-float, down from 7x-8x behind. | A | ✅ Holds up |
| [**zmij (Żmij float-to-string)**](dtolnay-zmij.md) | Rust port of Victor Zverovich's Żmij float printer: 8x-11x faster than std's float Display and about 4x faster than ryu. | A | ✅ Holds up |
| [**lexical-core (number parsing and formatting)**](alexhuszagh-rust-lexical.md) | Configurable number parsing and formatting; 2.8x faster than std at printing floats, but no longer the fastest and only 1.1x-1.5x ahead at parsing. | A | 🟡 Holds up in part |
| [**itoa (integer-to-string)**](dtolnay-itoa.md) | Integer-to-decimal formatting without core::fmt::Formatter; std 1.98's NumBuffer::format_into now offers the same path built in. | D | 🟡 Holds up in part |
| [**fast-float (Eisel-Lemire float parsing)**](aldanor-fast-float-rust.md) | Port of Lemire's fast_float that beat std's float parsing 2-8x on Rust 1.49; std absorbed the algorithm in 1.55 and the gap is now 1.0x-1.3x. | A | ❌ Doesn't hold up |
