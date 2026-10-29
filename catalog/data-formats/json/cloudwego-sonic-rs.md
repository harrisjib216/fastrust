# sonic-rs (SIMD JSON parser and serializer)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md) › [JSON](README.md)</sub>

**Tier A** (reproducible public benchmark) · 🟡 Holds up in part · Open source · `Rust` · `Apache-2.0` · 🟢 active

> ByteDance's SIMD JSON library; its own runs parse 1.7x-2.3x (structs) and 3.4x-5.2x (untyped) faster than serde_json with target-cpu=native.

| | |
|---|---|
| **Beats** | serde_json::from_str / to_string (serde_json version at the time of the README run, not stated) |
| **Claimed result** | Struct deserialize: twitter 827.74 µs vs serde_json::from_str 1.3842 ms, citm_catalog 1.3671 ms vs 2.6079 ms, canada 4.0212 ms vs 9.2563 ms (1.67x-2.30x). Untyped: twitter 556.10 µs vs 2.8960 ms, citm_catalog 1.6981 ms vs 8.0706 ms, canada 4.9568 ms vs 16.640 ms (3.36x-5.21x). Struct serialize: twitter 448.25 µs vs 739.97 µs (1.65x), canada 4.7481 ms vs 4.6281 ms (sonic-rs slower) (ratios derived from page data, criterion medians) |
| **Workload** | json-benchmark files twitter.json, citm_catalog.json and canada.json; parse into the json-benchmark Rust structs or into an untyped document, and serialize back; UTF-8 validation on, serde_json with float_roundtrip. |
| **Metric** | time |
| **Hardware** | Intel Xeon Platinum 8260 @ 2.40GHz |
| **Toolchain** | -C target-cpu=native (repo .cargo/config.toml, also required by the README); rustc version not stated |
| **Adoption** | 943 stars |
| **Last commit** | 2026-10-11 |
| **Platforms** | x86_64 and aarch64 SIMD paths; other architectures use a fallback the README calls possibly very slow |
| **Install** | cargo add sonic-rs (build with RUSTFLAGS="-C target-cpu=native") |
| **Links** | [repository](https://github.com/cloudwego/sonic-rs) · [documentation](https://docs.rs/sonic-rs) · [benchmark](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md) |

## Verdict

The README numbers show struct parsing 1.67x-2.30x and untyped parsing 3.36x-5.21x faster than serde_json::from_str, but they need -C target-cpu=native, serialization is mixed (canada struct serialization is slightly slower than serde_json), and no independent benchmark has reproduced them. The run predates serde_json's 2025 switch to a faster float formatter.

**Why tier A:** First-party, but the benchmark code is public (cargo bench targets), the CPU is named and the required -C target-cpu=native flag is stated.

## Use it when

- Large JSON documents parsed into structs or untyped values on x86_64 or aarch64, built for the deployment CPU
- Pulling a few fields out of big documents with get/LazyValue instead of parsing everything
- String-heavy serialization such as the twitter.json case

## Skip it when

- You ship portable binaries without -C target-cpu=native: that configuration was not measured
- Architectures other than x86_64 and aarch64
- Number-heavy serialization (canada.json), where serde_json matched or beat it
- Code that needs serde_json::Value semantics such as hash-map object lookup

## How it is fast

Parses straight into the target struct, with no intermediate tape as in simd-json. SIMD is used for long strings, float fractions, whitespace skipping and skipping unneeded fields. Untyped documents live in one arena and objects are stored as arrays, not hash maps.

## What it costs

- target-cpu=native binaries are not portable across x86 CPUs
- get_unchecked and lazy iterator APIs are unsafe and give unspecified results on invalid JSON
- The sanitize feature, needed under LLVM sanitizers, costs about 30% in serialization per the README

## Problems with the evidence

- First-party numbers only; sonic-rs is not in rust_serialization_benchmark or json-benchmark
- No run date or crate versions on the page; git history shows the x86 struct-parsing numbers first appeared on 2023-10-23, so the serde_json compared is roughly three years old
- In this run serde_json::from_slice was slower than from_str, so ratios against from_slice would overstate the gap; this record uses from_str
- Single machine, criterion medians, no variance analysis beyond criterion intervals

## Evidence

<details>
<summary>19 quotes from the sources, checked 2026-10-28</summary>

> Model name:          Intel(R) Xeon(R) Platinum 8260 CPU @ 2.40GHz
>
> — [github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be34…](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md)

> Please add the compile options `-C target-cpu=native`
>
> — [github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be34…](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md)

> twitter/sonic_rs::from_slice time: [796.44 µs 827.74 µs 861.30 µs]
>
> — [github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be34…](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md)

> twitter/serde_json::from_str time: [1.3504 ms 1.3842 ms 1.4246 ms]
>
> — [github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be34…](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md)

> citm_catalog/sonic_rs::from_slice time: [1.3344 ms 1.3671 ms 1.4050 ms]
>
> — [github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be34…](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md)

> citm_catalog/serde_json::from_str time: [2.5736 ms 2.6079 ms 2.6518 ms]
>
> — [github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be34…](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md)

> canada/sonic_rs::from_slice time: [3.9676 ms 4.0212 ms 4.0906 ms]
>
> — [github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be34…](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md)

> canada/serde_json::from_str time: [9.0383 ms 9.2563 ms 9.5048 ms]
>
> — [github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be34…](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md)

> twitter/sonic_rs_dom::from_slice time: [550.95 µs 556.10 µs 562.89 µs]
>
> — [github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be34…](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md)

> twitter/serde_json::from_str time: [2.8618 ms 2.8960 ms 2.9396 ms]
>
> — [github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be34…](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md)

> canada/sonic_rs_dom::from_slice time: [4.9060 ms 4.9568 ms 5.0213 ms]
>
> — [github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be34…](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md)

> canada/serde_json::from_str time: [16.398 ms 16.640 ms 16.932 ms]
>
> — [github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be34…](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md)

> citm_catalog/sonic_rs_dom::from_slice time: [1.6657 ms 1.6981 ms 1.7341 ms]
>
> — [github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be34…](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md)

> citm_catalog/serde_json::from_str time: [7.8652 ms 8.0706 ms 8.3074 ms]
>
> — [github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be34…](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md)

> twitter/sonic_rs::to_string time: [434.03 µs 448.25 µs 463.97 µs]
>
> — [github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be34…](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md)

> twitter/serde_json::to_string time: [719.70 µs 739.97 µs 762.69 µs]
>
> — [github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be34…](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md)

> canada/sonic_rs::to_string time: [4.6701 ms 4.7481 ms 4.8404 ms]
>
> — [github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be34…](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md)

> canada/serde_json::to_string time: [4.5708 ms 4.6281 ms 4.6967 ms]
>
> — [github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be34…](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md)

> Don't enable this feature in production, since it will cause 30% performance loss in serialize.
>
> — [github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be34…](https://github.com/cloudwego/sonic-rs/blob/1643b38e331cee2348bf2cb01476be3458516550/README.md)

</details>
