# Random numbers

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Crypto and hashing](../README.md)</sub>

**The default:** rand's thread_rng and StdRng.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**fastrand (wyrand PRNG)**](smol-rs-fastrand.md) | Small wyrand-based RNG: 0.51 ns per u64 against 4.11 ns for rand's StdRng and 1.13 ns for SmallRng on an M1 Max. | B | ✅ Holds up |
| [**rand SmallRng (Xoshiro256++ on 64-bit)**](rust-random-rand-smallrng.md) | rand's non-cryptographic SmallRng: about 2.7x to 3.6x faster than the ChaCha12 generator behind StdRng and rand::rng(). | B | ✅ Holds up |
