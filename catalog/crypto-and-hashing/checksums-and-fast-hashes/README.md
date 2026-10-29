# Checksums and fast hashes

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Crypto and hashing](../README.md)</sub>

**The default:** crc32 and std's DefaultHasher.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**crc-fast (SIMD CRC-16/32/64 for all catalogued variants)**](awesomized-crc-fast-rust.md) | SIMD CRC for every CRC-16/32/64 variant, reaching ~111 GiB/s on AVX-512 servers; the '>220X' headline compares against crc's slowest default. | C | 🟡 Holds up in part |
| [**crc32fast (SIMD CRC-32/IEEE)**](srijs-rust-crc32fast.md) | CRC-32/IEEE with runtime-selected PCLMULQDQ or ARMv8 CRC instructions; 7.3 GB/s in its own table, far ahead of table-driven CRC. | C | 🟡 Holds up in part |
