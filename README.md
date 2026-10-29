<h1 align="center">fastrust</h1>

<!-- fastrust:badges -->
<p align="center">
  <img alt="entries: 134" src="https://img.shields.io/badge/entries-134-2ea44f">
  <img alt="tier A: 61" src="https://img.shields.io/badge/tier_A-61-1f6feb">
  <img alt="categories: 37" src="https://img.shields.io/badge/categories-37-8250df">
  <img alt="claims rejected: 16" src="https://img.shields.io/badge/claims_rejected-16-cf222e">
  <img alt="license: MIT" src="https://img.shields.io/badge/license-MIT-blue">
</p>
<!-- /fastrust:badges -->

<p align="center"><b>Find something faster than the default you reach for in Rust, along with the benchmark, the default it beats, and the catch.</b></p>

**fastrust** is a catalog of Rust crates, standard-library changes and build settings that claim to be faster than
the defaults Rust developers use: serde_json, std's HashMap and its SipHash hasher, Mutex, mpsc, Tokio, the system
allocator, flate2, reqwest, the default linker, `cargo build` and others. Every entry names what it beats, quotes the
published numbers, grades how far those numbers can be trusted, and says what goes wrong if you adopt it. Claims that
don't hold up stay in the catalog, marked as rejected, because knowing what not to switch to is half the value.

It is the Rust edition of [fastplusplus](https://github.com/harrisjib216/fastplusplus) (C++) and
[fastswift](https://github.com/harrisjib216/fastswift) (Swift).

<!-- fastrust:stats -->
**134 entries** across **37 categories** in **10 domains** · 61 tier A · 30 tier B · 29 tier C · 14 tier D · 50 hold up · 68 hold up in part · 16 rejected
<!-- /fastrust:stats -->

## Find something faster

<!-- fastrust:index -->
| Domain | Entries | Categories |
|---|--:|---|
| [**Data formats**](catalog/data-formats/README.md) | 22 | [JSON](catalog/data-formats/json/README.md) 4 · [Binary serialization](catalog/data-formats/binary-serialization/README.md) 5 · [Base64, hex and encoding](catalog/data-formats/base64-and-encoding/README.md) 5 · [Numbers to and from text](catalog/data-formats/numbers-to-and-from-text/README.md) 6 · [Parsers and CSV](catalog/data-formats/parsers-and-csv/README.md) 2 |
| [**Collections**](catalog/collections/README.md) | 21 | [Hashers for maps and sets](catalog/collections/hashers/README.md) 5 · [Maps and sets](catalog/collections/maps-and-sets/README.md) 3 · [Small vectors and strings](catalog/collections/small-vectors-and-strings/README.md) 4 · [Sorting and searching](catalog/collections/sorting-and-searching/README.md) 4 · [Concurrent maps](catalog/collections/concurrent-maps/README.md) 5 |
| [**Concurrency and runtime**](catalog/concurrency-and-runtime/README.md) | 17 | [Locks](catalog/concurrency-and-runtime/locks/README.md) 3 · [Channels](catalog/concurrency-and-runtime/channels/README.md) 4 · [Async runtimes](catalog/concurrency-and-runtime/async-runtimes/README.md) 4 · [Data parallelism](catalog/concurrency-and-runtime/parallelism/README.md) 1 · [Lazy statics and one-time init](catalog/concurrency-and-runtime/lazy-and-once/README.md) 1 · [Memory allocators](catalog/concurrency-and-runtime/allocators/README.md) 4 |
| [**Text processing**](catalog/text-processing/README.md) | 8 | [Regular expressions](catalog/text-processing/regex/README.md) 3 · [String and byte search](catalog/text-processing/string-search/README.md) 3 · [UTF-8 and Unicode](catalog/text-processing/utf8-and-unicode/README.md) 2 |
| [**Crypto and hashing**](catalog/crypto-and-hashing/README.md) | 8 | [Cryptographic hashes and ciphers](catalog/crypto-and-hashing/crypto-hashes-and-ciphers/README.md) 4 · [Checksums and fast hashes](catalog/crypto-and-hashing/checksums-and-fast-hashes/README.md) 2 · [Random numbers](catalog/crypto-and-hashing/random-numbers/README.md) 2 |
| [**Compression**](catalog/compression/README.md) | 8 | [Deflate, zlib and gzip](catalog/compression/deflate-and-gzip/README.md) 4 · [Zstandard, LZ4, Snappy and Brotli](catalog/compression/other-codecs/README.md) 4 |
| [**Storage**](catalog/storage/README.md) | 12 | [Embedded databases](catalog/storage/embedded-databases/README.md) 5 · [SQL drivers and ORMs](catalog/storage/sql-drivers/README.md) 4 · [In-memory caches](catalog/storage/caches/README.md) 3 |
| [**Server and networking**](catalog/server-and-networking/README.md) | 10 | [Web frameworks](catalog/server-and-networking/web-frameworks/README.md) 5 · [HTTP clients](catalog/server-and-networking/http-clients/README.md) 1 · [Protocol Buffers and RPC](catalog/server-and-networking/protobuf-and-rpc/README.md) 4 |
| [**Numerics and media**](catalog/numerics-and-media/README.md) | 9 | [Linear algebra](catalog/numerics-and-media/linear-algebra/README.md) 2 · [SIMD](catalog/numerics-and-media/simd/README.md) 3 · [Images](catalog/numerics-and-media/images/README.md) 4 |
| [**Build and tooling**](catalog/tooling/README.md) | 19 | [Linkers](catalog/tooling/linkers/README.md) 6 · [Compile times](catalog/tooling/compile-times/README.md) 5 · [Build settings for faster binaries](catalog/tooling/runtime-build-settings/README.md) 6 · [Test runners and build caching](catalog/tooling/test-runners-and-caching/README.md) 2 |
<!-- /fastrust:index -->

Browse everything from the [catalog index](catalog/README.md), or query the export,
[`catalog/catalog.jsonl`](catalog/catalog.jsonl) (one entry per line, [schema](docs/schema.md)).

## Highlights

The strongest entry in each domain whose claim holds up as stated.

<!-- fastrust:featured -->
| Need faster… | Pick | What it is | Tier |
|---|---|---|:-:|
| [Binary serialization](catalog/data-formats/binary-serialization/README.md) | [**rkyv (zero-copy deserialization)**](catalog/data-formats/binary-serialization/rkyv-rkyv.md) | Zero-copy archive format: reads data in place in nanoseconds instead of deserializing, and serializes plain-data arrays 35x faster than bincode 1.3. | **A** |
| [Concurrent maps](catalog/collections/concurrent-maps/README.md) | [**DashMap (sharded RwLock hash map)**](catalog/collections/concurrent-maps/xacrimon-dashmap.md) | Sharded concurrent HashMap with a HashMap-like &self API; scales far past RwLock<HashMap> under contention but is slower single-threaded. | **A** |
| [Memory allocators](catalog/concurrency-and-runtime/allocators/README.md) | [**snmalloc-rs (snmalloc global allocator)**](catalog/concurrency-and-runtime/allocators/microsoft-snmalloc.md) | Rust bindings for Microsoft's message-passing allocator; 7.2x faster than glibc on cross-thread frees and 4-18% faster on real programs. | **A** |
| [UTF-8 and Unicode](catalog/text-processing/utf8-and-unicode/README.md) | [**simdutf8 (SIMD UTF-8 validation)**](catalog/text-processing/utf8-and-unicode/rusticstuff-simdutf8.md) | SIMD drop-in for std::str::from_utf8: up to 23x faster on valid non-ASCII text and up to 4x on ASCII for inputs of 64 bytes or more. | **A** |
| [Cryptographic hashes and ciphers](catalog/crypto-and-hashing/crypto-hashes-and-ciphers/README.md) | [**ring (BoringSSL-derived crypto primitives)**](catalog/crypto-and-hashing/crypto-hashes-and-ciphers/briansmith-ring.md) | Rust API over BoringSSL's assembly: about 3x RustCrypto's AES-GCM throughput and 8x its ECDSA P-256 signing rate on a Ryzen 9 7940HS. | **B** |
| [Deflate, zlib and gzip](catalog/compression/deflate-and-gzip/README.md) | [**zlib-rs (flate2 zlib-rs backend)**](catalog/compression/deflate-and-gzip/trifectatechfoundation-zlib-rs.md) | Rust port of zlib-ng as a flate2 backend; inflates about 1.3x and deflates 1.9x to 2.6x faster than flate2's default miniz_oxide. | **A** |
| [SQL drivers and ORMs](catalog/storage/sql-drivers/README.md) | [**rusqlite (synchronous SQLite driver)**](catalog/storage/sql-drivers/rusqlite-rusqlite.md) | Synchronous SQLite bindings; queries ran 11x to 49x faster than sqlx's async SQLite driver in Diesel's nightly suite. | **A** |
| [Images](catalog/numerics-and-media/images/README.md) | [**fast_image_resize (SIMD image resizing)**](catalog/numerics-and-media/images/cykooz-fast-image-resize.md) | SIMD image resizer that downsizes 5-27x faster than the image crate's resize, depending on filter, pixel format and CPU. | **A** |
| [Linkers](catalog/tooling/linkers/README.md) | [**mold (ELF linker)**](catalog/tooling/linkers/rui314-mold.md) | Parallel ELF linker by lld's original author; links large programs several times faster than lld and far faster than GNU ld. | **A** |
<!-- /fastrust:featured -->

## The default caught up

Rust's standard library and toolchain keep absorbing the crates that beat them. A crate that was twice as fast as std
in 2019 may be running the same algorithm as std today, so every entry records when, and against which version, it
was measured.

- **Locks.** [parking_lot](catalog/concurrency-and-runtime/locks/amanieu-parking-lot.md) was 1.5x faster than std's
  Mutex uncontended and up to 5x contended, when std wrapped pthread mutexes. Rust 1.62
  [moved std's locks to futexes](catalog/concurrency-and-runtime/locks/std-sync-mutex.md) on Linux, and three
  lock-bench runs since have parking_lot 3.6x to 11x *slower* under extreme contention.
- **Channels.** Rust 1.67 [replaced std's mpsc](catalog/concurrency-and-runtime/channels/std-sync-mpsc.md) with a
  copy of [crossbeam-channel](catalog/concurrency-and-runtime/channels/crossbeam-rs-crossbeam-crossbeam-channel.md).
  What crossbeam still adds is features, not speed, and [flume](catalog/concurrency-and-runtime/channels/zesterer-flume.md)'s
  "always faster than std::sync::mpsc" was charted against the old one.
- **Hash maps.** [hashbrown](catalog/collections/maps-and-sets/rust-lang-hashbrown.md) has been
  [std's HashMap](catalog/collections/maps-and-sets/std-collections-hashmap.md) since Rust 1.36. Its README's "around
  2x faster" is against the table std had before that. The difference left is the hasher: hashbrown defaults to
  [foldhash](catalog/collections/hashers/orlp-foldhash.md), about 4.6x to 5x faster than std's SipHash in
  hashing-heavy benchmarks, without SipHash's resistance to HashDoS.
- **Sorting.** Rust 1.81 made [`sort`](catalog/collections/sorting-and-searching/std-slice-sort.md) driftsort and
  [`sort_unstable`](catalog/collections/sorting-and-searching/std-slice-sort-unstable.md) ipnsort, which narrows
  the leads [glidesort](catalog/collections/sorting-and-searching/orlp-glidesort.md) and radix sorts published
  against the old ones.
- **Float parsing.** [fast-float](catalog/data-formats/numbers-to-and-from-text/aldanor-fast-float-rust.md) was 2x to
  8x faster than `str::parse` on Rust 1.49. Rust 1.55 merged its algorithm into std, which now trails it by
  1.02x to 1.29x and beats it on large integer-valued floats.
- **Linking.** Since Rust 1.90, [rustc links with lld](catalog/tooling/linkers/rustc-lld-default-linux.md) on
  x86_64 Linux. On macOS, Apple's ld-prime is faster than [lld](catalog/tooling/linkers/llvm-llvm-project-lld-macho.md),
  so the alternative linkers that beat Apple's old ld64 no longer help there.
- **JPEG.** The [image crate](catalog/numerics-and-media/images/image-rs-image-zune-jpeg-decoder.md) has decoded JPEG
  with zune-jpeg since 0.25, so adding [zune-jpeg](catalog/numerics-and-media/images/etemesi254-zune-image-zune-jpeg.md)
  yourself no longer beats the default.

Every change to Rust's own defaults in the catalog, newest release first:

<!-- fastrust:caught-up -->
| Release | Change | What it does | Tier |
|---|---|---|:-:|
| 1.90 | [**rust-lld as the default linker on x86_64 Linux (Rust 1.90)**](catalog/tooling/linkers/rustc-lld-default-linux.md) | Since Rust 1.90, rustc links x86_64 Linux programs with its bundled lld instead of GNU ld, which cuts link time several-fold. | B |
| 1.81 | [**std slice::sort (driftsort, Rust 1.81)**](catalog/collections/sorting-and-searching/std-slice-sort.md) | Since Rust 1.81, slice::sort is driftsort: about 2.4x faster on random u64 and far faster on inputs with many duplicates. | A |
| 1.81 | [**std slice::sort_unstable (ipnsort, Rust 1.81)**](catalog/collections/sorting-and-searching/std-slice-sort-unstable.md) | Since Rust 1.81, slice::sort_unstable is ipnsort: a faster, smaller pdqsort successor with a sorting-network small-sort. | A |
| 1.67 | [**std str::contains SSE2 fast path for short needles (Rust 1.67)**](catalog/text-processing/string-search/std-str-contains-sse2.md) | Rust 1.67 made str::contains with short needles 6x to 9x faster in long-haystack benchmarks on x86_64 by replacing Two-Way with an SSE2 scan. | B |
| 1.67 | [**std::sync::mpsc (crossbeam-channel implementation, Rust 1.67)**](catalog/concurrency-and-runtime/channels/std-sync-mpsc.md) | Since Rust 1.67 std::sync::mpsc wraps a copy of crossbeam-channel, closing most of the gap that made crossbeam the default recommendation. | D |
| 1.62 | [**std::sync::Mutex and RwLock (futex-based, Rust 1.62)**](catalog/concurrency-and-runtime/locks/std-sync-mutex.md) | Since Rust 1.62 std's Mutex, RwLock and Condvar on Linux are futex-based; the Mutex ran 2-4x faster than the pthread one it replaced in lock-bench. | B |
| 1.60 | [**std str::chars().count() word-at-a-time counting (Rust 1.60)**](catalog/text-processing/utf8-and-unicode/std-str-chars-count.md) | Rust 1.60 made str::chars().count() about 4x to 5x faster on medium and large strings by counting a machine word at a time. | B |
| 1.55 | [**std float parsing (Eisel-Lemire, Rust 1.55)**](catalog/data-formats/numbers-to-and-from-text/std-num-f64-from-str.md) | Since Rust 1.55, str::parse::<f64> uses Eisel-Lemire and runs within about 1.2x of fast-float, down from 7x-8x behind. | A |
| 1.45 | [**std Vec growth skips capacities 1 and 2 (Rust 1.45)**](catalog/collections/small-vectors-and-strings/std-vec-min-non-zero-cap.md) | Since Rust 1.45, a Vec grown by push starts at capacity 4 (8 for bytes) instead of 1, removing two reallocations for small Vecs. | B |
| 1.38 | [**Pipelined compilation in Cargo (Rust 1.38)**](catalog/tooling/compile-times/rustc-pipelined-compilation.md) | Since Rust 1.38, Cargo starts compiling a crate as soon as its dependencies' metadata is ready, not after they finish. | B |
| 1.36 | [**std HashMap and HashSet (hashbrown SwissTable, Rust 1.36)**](catalog/collections/maps-and-sets/std-collections-hashmap.md) | Since Rust 1.36, std's HashMap is the hashbrown SwissTable, about 2x faster on lookups than the old Robin Hood table with the same hasher. | C |
<!-- /fastrust:caught-up -->

## Claims that don't hold up

<!-- fastrust:rejected -->
| Claim | Why it doesn't hold up |
|---|---|
| [**arrow-rs `simd` feature (hand-written packed_simd arithmetic kernels)**](catalog/numerics-and-media/simd/apache-arrow-rs-simd-feature.md) | Arrow's own measurements showed the scalar, auto-vectorized add, subtract and multiply kernels running 1.5x to 2.1x faster than its explicit SIMD versions, so the project deleted them in 2022 and removed the whole simd feature in December 2023. Explicit SIMD kept a lead only for checked division, where the scalar loop has a branch per element. |
| [**borsh (Binary Object Representation Serializer for Hashing)**](catalog/data-formats/binary-serialization/near-borsh-rs.md) | borsh.io says skipping serde makes borsh faster than bincode in some cases. In the independent 2026-08-07 run borsh 1.8.0 was within 11% of bincode 1.3.3 everywhere: slower on log and mesh, and 4%-7% faster only on minecraft_savedata. That is not a measurable speed advantage; borsh's real selling point is a canonical, hash-stable encoding. |
| [**fast-float (Eisel-Lemire float parsing)**](catalog/data-formats/numbers-to-and-from-text/aldanor-fast-float-rust.md) | The 2-8x lead over from_str was measured on Rust 1.49. Rust 1.55 merged a port of this crate's algorithm into std, and on Rust 1.81 and 1.83 fast-float is only 1.02x-1.29x faster on five datasets and 1.32x-1.60x slower than std on large integer-valued floats. The crate is abandoned with two RustSec advisories; fast-float2 is the patched fork for byte-slice and partial parsing. |
| [**json-rust (json crate, DOM parser)**](catalog/data-formats/json/maciejhirsz-json-rust.md) | The README says it can even outperform parsing to structs, but in the benchmark it links (2021 run) json-rust DOM parsing at 390-520 MB/s was slower than serde_json struct parsing at 550-710 MB/s on all three files. Its lead over serde_json's Value was real but is five years old, and the crate has had no release since 2020, carries an unmaintained advisory with an open soundness report, and was dropped from that benchmark in 2023. |
| [**LLVM lld for macOS (ld64.lld)**](catalog/tooling/linkers/llvm-llvm-project-lld-macho.md) | An older Chromium doc (2021 or earlier) reported lld 4x faster than Apple's old ld64, but Apple replaced ld64 with ld-prime in Xcode 15. Chromium's current docs say ld-prime is faster than lld for local arm64 macOS builds, by up to 4-6x, and Bevy's build config now keeps the default macOS linker for the same reason. |
| [**once_cell (sync::Lazy)**](catalog/concurrency-and-runtime/lazy-and-once/matklad-once-cell.md) | The only speed claim found, 'about 20% faster in a 32-thread test', links to a once_cell example with no published results that is no longer in the repository. The same page's own benchmark measured 26 vs 27 ns per iteration, within noise. No measured speed difference separates lazy_static, once_cell and std's OnceLock (1.70) and LazyLock (1.80). |
| [**parking_lot (Mutex and RwLock)**](catalog/concurrency-and-runtime/locks/amanieu-parking-lot.md) | The '1.5x faster uncontended, up to 5x faster contended' claim dates from when std wrapped pthread mutexes. Against the futex-based std Mutex (Rust 1.62+), three lock-bench runs from 2022 to 2025 have parking_lot 3.6x to 11x slower under extreme contention, with only a 13-27% edge at light or no contention. The author removed the speed claim from the README in October 2026. |
| [**pcre2 (Rust bindings to PCRE2 with JIT)**](catalog/text-processing/regex/burntsushi-rust-pcre2.md) | In the December 2025 rebar run, PCRE2's JIT has a search geometric mean of 6.00 against 3.08 for the regex crate, so it is slower overall. It wins specific workloads (unstructured-to-json, Unicode character data, some case-insensitive literals) and loses badly on others, such as the Cloudflare ReDoS patterns. |
| [**RustCrypto asm-hashes (the sha2 'asm' feature)**](catalog/crypto-and-hashing/crypto-hashes-and-ciphers/rustcrypto-asm-hashes.md) | The asm feature was removed in sha2 0.11.0 and the asm-hashes repository is archived. Its maintainers only ever described it as 'a bit faster' than the software backend, and sha2 already uses SHA-NI and ARMv8 SHA2 instructions at runtime, which is where large gains come from. |
| [**ruzstd (pure-Rust zstd)**](catalog/compression/other-codecs/killingspark-zstd-rs.md) | No pure-Rust zstd beats the C bindings. ruzstd's own README puts its decoder 1.4x to 3.5x slower, its encoder lacks the default level, and Trifecta's newer libzstd-rs-sys port is about 3% slower than C by default. |
| [**sled (Bw-tree-style embedded KV store)**](catalog/storage/embedded-databases/spacejam-sled.md) | sled claims LSM-like write speed with B+tree-like reads. Against an actual LSM, its batch writes were 1.9x slower than RocksDB and 2.4x slower than fjall, and its bulk load was the slowest tested. Its reads beat RocksDB but trail redb and LMDB. fjall's author excluded it from durable-write tests because it does not reliably fsync, and redb dropped it over open security advisories and no recent release. |
| [**smartstring (SmartString inline string)**](catalog/collections/small-vectors-and-strings/bodil-smartstring.md) | The allocation savings are real (19 allocation events against 2,017 for String in an independent 2020 test), but the README's claim that inlining makes strings faster overall is contradicted by the current independent string benchmark, which leaves smartstring out of its access test because it is too slow. The repository is archived. |
| [**sold (mold fork for macOS)**](catalog/tooling/linkers/bluewhalesystems-sold.md) | The author archived sold in November 2024 and now recommends Apple's new linker in Xcode 15 or later instead. Bevy's build guide says it is about the same speed as the default macOS linker. No published numbers show a gain over today's default. |
| [**ureq (blocking HTTP client)**](catalog/server-and-networking/http-clients/algesten-ureq.md) | ureq does not claim to be faster than reqwest, and the one public head-to-head shows no difference: 108.4 ms vs 108.7 ms for 1,000 requests, inside each other's error bars. Choose ureq for its small blocking API and dependency tree, not for speed. |
| [**zld (faster fork of Apple ld64)**](catalog/tooling/linkers/michaeleisel-zld.md) | zld claimed links at least 40% faster than ld64, but it was archived in February 2023 and its author points to lld instead. Apple replaced ld64 with ld-prime in Xcode 15, and Chromium now measures ld-prime as faster than lld, so the baseline zld beat is gone. |
| [**zune-jpeg (JPEG decoder)**](catalog/numerics-and-media/images/etemesi254-zune-image-zune-jpeg.md) | The 1.5x-2x lead over jpeg-decoder was real enough that the image crate adopted zune-jpeg as its JPEG decoder in 0.25.0 (March 2024). Anyone decoding JPEG through image 0.25 already uses zune-jpeg, so adding it directly no longer beats the default. |
<!-- /fastrust:rejected -->

## What an entry tells you

Each entry has its own page with the same parts:

- **Beats.** The specific default, with its version where that matters ("std's Mutex before the 1.62 futex
  rewrite").
- **Claimed result.** The published numbers, quoted, and the workload, metric, hardware, OS and toolchain behind
  them, when the source states them.
- **Verdict.** *Holds up*, *holds up in part* (a narrower workload, a smaller gap, an old rustc, or only with
  `target-cpu=native`), or *doesn't hold up* (newer data contradicts it, the baseline was misrepresented, or the
  "faster" option is slower).
- **Use it when, skip it when, how it is fast, what it costs.** Costs include `unsafe` code, lost HashDoS
  resistance, compile time, binary size and MSRV.
- **Problems with the evidence.** Debug builds, old toolchains, microbenchmarks, tiny samples, numbers that exist
  only in a chart image, benchmarks run by a competitor.
- **Evidence.** The quotes the numbers come from, each linked to its source.

### Evidence tiers

| Tier | Meaning |
|:-:|---|
| **A** | Public benchmark code that someone else can run, with the toolchain or flags and the hardware stated. |
| **B** | Independent of the crate's authors: a third-party benchmark, a paper, a talk with numbers, or the Rust project measuring its own change. |
| **C** | First-party numbers without enough detail to reproduce. |
| **D** | Directional: no numbers in text, or numbers only in a chart image. |

The tier grades the evidence, not the crate. A tier D entry can be the right choice; it just hasn't been measured in
a way you can check.

## How it's built

Each entry is a JSON record in [`data/entries/`](data/entries). The Rust tool in [`tools/`](tools) does the rest:

```sh
cargo run -p catalog -- check    # every record against the schema
cargo run -p catalog -- verify   # fetch every source and find every quote on it
cargo run -p catalog -- render   # write catalog/, the exports, and this README's tables
```

`check` also makes sure every number in an entry's claimed result appears in one of its quotes; only ratios and
clauses marked "derived from page data" are exempt. `verify` fetches each evidence URL again (GitHub files from
their raw source) and checks that every quote is on the page, ignoring case and whitespace. A quote that has been
paraphrased, or that a page has since dropped, fails the check.

## Contributing

Corrections are the most useful contribution: a newer benchmark that changes a verdict, a default that caught up, or
a number that's wrong. See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT. The catalog quotes published benchmarks for commentary. Each quote links to its source.
