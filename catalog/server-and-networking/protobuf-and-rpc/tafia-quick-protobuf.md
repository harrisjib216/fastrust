# quick-protobuf (borrowing protobuf decoder)

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Server and networking](../README.md) › [Protocol Buffers and RPC](README.md)</sub>

**Tier C** (first-party numbers) · 🟡 Holds up in part · Open source · `Rust` · `MIT` · 🟡 stale

> Protobuf crate whose generated types borrow strings and bytes from the input; 2018 numbers show faster reads than prost, slower writes.

| | |
|---|---|
| **Beats** | prost (version at the time of the 2018 results; the perftest manifest now names prost 0.9.0) |
| **Claimed result** | ns/iter, quick-protobuf vs prost reads: test1 31 vs 53, test_strings 185 vs 247, test_small_bytearrays 99 vs 729, test_large_bytearrays 8598 vs 66959 (1.71x, 1.34x, 7.36x, 7.79x) (derived from page data). prost faster on writes: test_optional_messages 104 vs 157, test_repeated_packed_int32 46 vs 69, test_small_bytearrays 318 vs 463 ns/iter |
| **Workload** | rust-protobuf's perftest messages adapted for three crates: small scalar messages, repeated bools and packed int32, nested and optional messages, strings, small and large byte arrays; write and read per message set. |
| **Metric** | time |
| **Measured** | 2018 |
| **Adoption** | 473 stars |
| **Last commit** | 2024-02-14 |
| **Platforms** | Any; no_std support |
| **Install** | cargo add quick-protobuf |
| **Links** | [repository](https://github.com/tafia/quick-protobuf) · [documentation](https://docs.rs/quick-protobuf) · [benchmark](https://github.com/tafia/quick-protobuf/tree/master/perftest) |

## Verdict

quick-protobuf reads strings and byte fields 1.3x to 7.8x faster than prost by borrowing from the input, but prost encodes faster in several tests, and the only published numbers date from 2018. Since then prost can decode bytes fields into zero-copy bytes::Bytes and has had years of optimization, while quick-protobuf has had no commit since February 2024.

**Why tier C:** First-party numbers from the crate's own perftest; the benchmark code is public but no hardware, rustc version or date is given next to the results.

## Use it when

- Messages are mostly strings or large byte fields, you decode far more than you encode, and the input buffer outlives the decoded message.

## Skip it when

- You use tonic: it is built around prost-generated types.
- Encoding speed matters; prost was faster on most write tests even in 2018.
- You need current protobuf features (editions, well-known types, reflection) or an actively maintained code generator.

## How it is fast

pb-rs generates structs with Cow<str> and Cow<[u8]> fields that point into the input buffer, so reading a message does not allocate for strings and byte arrays. Packed fixed-size repeated fields can be read without copying. The gain grows with payload size, which is why the large byte-array read shows the biggest gap.

## What it costs

- Generated types carry a lifetime tied to the input buffer.
- Separate code generator (pb-rs) and a much smaller user base than prost.
- No commits since 2024-02-14.

## Problems with the evidence

- The results block in perftest/README.md was last changed in 2018 according to the git history, while the manifest was later bumped to prost 0.9.0 without rerunning; the prost version actually measured is unknown.
- No hardware, rustc version or flags are given.
- prost can now decode bytes fields as bytes::Bytes, zero-copy slices of the input (buffa's 2026 tables measure it as "prost (bytes)"), which addresses the byte-array case; no newer quick-protobuf comparison exists.
- quick-protobuf is not in rust_serialization_benchmark, the main independent Rust serialization benchmark.

## Evidence

<details>
<summary>9 quotes from the sources, checked 2026-10-28</summary>

> labels rust-protobuf quick-protobuf prost quick/rust prost/rust ns/iter ns/iter ns/iter % %
>
> — [github.com/tafia/quick-protobuf/blob/54e7d6c5d981c6f7cec2e9a2167c10…](https://github.com/tafia/quick-protobuf/blob/54e7d6c5d981c6f7cec2e9a2167c10ed0f9392b4/perftest/README.md)

> test1 write 72 20 21 72.2 70.8 read 71 31 53 56.3 25.4
>
> — [github.com/tafia/quick-protobuf/blob/54e7d6c5d981c6f7cec2e9a2167c10…](https://github.com/tafia/quick-protobuf/blob/54e7d6c5d981c6f7cec2e9a2167c10ed0f9392b4/perftest/README.md)

> test_strings write 146 63 59 56.8 59.6 read 345 185 247 46.4 28.4
>
> — [github.com/tafia/quick-protobuf/blob/54e7d6c5d981c6f7cec2e9a2167c10…](https://github.com/tafia/quick-protobuf/blob/54e7d6c5d981c6f7cec2e9a2167c10ed0f9392b4/perftest/README.md)

> test_small_bytearrays write 479 463 318 3.3 33.6 read 747 99 729 86.7 2.4
>
> — [github.com/tafia/quick-protobuf/blob/54e7d6c5d981c6f7cec2e9a2167c10…](https://github.com/tafia/quick-protobuf/blob/54e7d6c5d981c6f7cec2e9a2167c10ed0f9392b4/perftest/README.md)

> test_large_bytearrays write 109299 96753 89523 11.5 18.1 read 47513 8598 66959 81.9 -40.9
>
> — [github.com/tafia/quick-protobuf/blob/54e7d6c5d981c6f7cec2e9a2167c10…](https://github.com/tafia/quick-protobuf/blob/54e7d6c5d981c6f7cec2e9a2167c10ed0f9392b4/perftest/README.md)

> test_optional_messages write 223 157 104 29.6 53.4
>
> — [github.com/tafia/quick-protobuf/blob/54e7d6c5d981c6f7cec2e9a2167c10…](https://github.com/tafia/quick-protobuf/blob/54e7d6c5d981c6f7cec2e9a2167c10ed0f9392b4/perftest/README.md)

> test_repeated_packed_int32 write 135 69 46 48.9 65.9
>
> — [github.com/tafia/quick-protobuf/blob/54e7d6c5d981c6f7cec2e9a2167c10…](https://github.com/tafia/quick-protobuf/blob/54e7d6c5d981c6f7cec2e9a2167c10ed0f9392b4/perftest/README.md)

> prost = "0.9.0"
>
> — [github.com/tafia/quick-protobuf/blob/54e7d6c5d981c6f7cec2e9a2167c10…](https://github.com/tafia/quick-protobuf/blob/54e7d6c5d981c6f7cec2e9a2167c10ed0f9392b4/perftest/Cargo.toml)

> in particular when working with string, bytes and repeated packed fixed size fields (no extra allocation)
>
> — [github.com/tafia/quick-protobuf](https://github.com/tafia/quick-protobuf)

</details>
