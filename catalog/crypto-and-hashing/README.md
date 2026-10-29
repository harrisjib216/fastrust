# Crypto and hashing

<sub>[fastrust](../../README.md) › [Catalog](../README.md)</sub>

## [Cryptographic hashes and ciphers](crypto-hashes-and-ciphers/README.md)

**The default:** The RustCrypto crates (sha2, aes-gcm and friends).

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**aws-lc-rs (AWS-LC with a ring-compatible API)**](crypto-hashes-and-ciphers/aws-aws-lc-rs.md) | Rust bindings to AWS-LC with ring's API: 3.2x RustCrypto's AES-GCM throughput and 10x its RSA-2048 signing rate on a Ryzen 9 7940HS. | B | ✅ Holds up |
| [**ring (BoringSSL-derived crypto primitives)**](crypto-hashes-and-ciphers/briansmith-ring.md) | Rust API over BoringSSL's assembly: about 3x RustCrypto's AES-GCM throughput and 8x its ECDSA P-256 signing rate on a Ryzen 9 7940HS. | B | ✅ Holds up |
| [**blake3 (BLAKE3 hash)**](crypto-hashes-and-ciphers/blake3-team-blake3.md) | BLAKE3 hash: 14x SHA-256 on a 2019 AVX-512 server without SHA-NI, but slower than hardware SHA-256 on short inputs. | A | 🟡 Holds up in part |
| [**RustCrypto asm-hashes (the sha2 'asm' feature)**](crypto-hashes-and-ciphers/rustcrypto-asm-hashes.md) | Assembly SHA-1/SHA-2/MD5 compression functions for RustCrypto; archived and dropped from sha2 0.11 with no published speedup. | D | ❌ Doesn't hold up |

## [Checksums and fast hashes](checksums-and-fast-hashes/README.md)

**The default:** crc32 and std's DefaultHasher.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**crc-fast (SIMD CRC-16/32/64 for all catalogued variants)**](checksums-and-fast-hashes/awesomized-crc-fast-rust.md) | SIMD CRC for every CRC-16/32/64 variant, reaching ~111 GiB/s on AVX-512 servers; the '>220X' headline compares against crc's slowest default. | C | 🟡 Holds up in part |
| [**crc32fast (SIMD CRC-32/IEEE)**](checksums-and-fast-hashes/srijs-rust-crc32fast.md) | CRC-32/IEEE with runtime-selected PCLMULQDQ or ARMv8 CRC instructions; 7.3 GB/s in its own table, far ahead of table-driven CRC. | C | 🟡 Holds up in part |

## [Random numbers](random-numbers/README.md)

**The default:** rand's thread_rng and StdRng.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**fastrand (wyrand PRNG)**](random-numbers/smol-rs-fastrand.md) | Small wyrand-based RNG: 0.51 ns per u64 against 4.11 ns for rand's StdRng and 1.13 ns for SmallRng on an M1 Max. | B | ✅ Holds up |
| [**rand SmallRng (Xoshiro256++ on 64-bit)**](random-numbers/rust-random-rand-smallrng.md) | rand's non-cryptographic SmallRng: about 2.7x to 3.6x faster than the ChaCha12 generator behind StdRng and rand::rng(). | B | ✅ Holds up |
