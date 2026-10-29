# Cryptographic hashes and ciphers

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Crypto and hashing](../README.md)</sub>

**The default:** The RustCrypto crates (sha2, aes-gcm and friends).

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**aws-lc-rs (AWS-LC with a ring-compatible API)**](aws-aws-lc-rs.md) | Rust bindings to AWS-LC with ring's API: 3.2x RustCrypto's AES-GCM throughput and 10x its RSA-2048 signing rate on a Ryzen 9 7940HS. | B | ✅ Holds up |
| [**ring (BoringSSL-derived crypto primitives)**](briansmith-ring.md) | Rust API over BoringSSL's assembly: about 3x RustCrypto's AES-GCM throughput and 8x its ECDSA P-256 signing rate on a Ryzen 9 7940HS. | B | ✅ Holds up |
| [**blake3 (BLAKE3 hash)**](blake3-team-blake3.md) | BLAKE3 hash: 14x SHA-256 on a 2019 AVX-512 server without SHA-NI, but slower than hardware SHA-256 on short inputs. | A | 🟡 Holds up in part |
| [**RustCrypto asm-hashes (the sha2 'asm' feature)**](rustcrypto-asm-hashes.md) | Assembly SHA-1/SHA-2/MD5 compression functions for RustCrypto; archived and dropped from sha2 0.11 with no published speedup. | D | ❌ Doesn't hold up |
