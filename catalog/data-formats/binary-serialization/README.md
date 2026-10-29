# Binary serialization

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md)</sub>

**The default:** bincode through serde, or serde_json as the wire or disk format.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**bitcode (compact binary Encode/Decode)**](softbearstudios-bitcode.md) | Binary format with its own derive that serializes 4x-6x and deserializes 1.4x-2.9x faster than bincode 1.3 via serde, with smaller output. | A | ✅ Holds up |
| [**rkyv (zero-copy deserialization)**](rkyv-rkyv.md) | Zero-copy archive format: reads data in place in nanoseconds instead of deserializing, and serializes plain-data arrays 35x faster than bincode 1.3. | A | ✅ Holds up |
| [**wincode (bincode-compatible serializer)**](anza-xyz-wincode.md) | Drop-in for bincode 1's wire format that serializes 3x-33x and deserializes 1.1x-26x faster than bincode 1.3.3 through serde. | A | ✅ Holds up |
| [**speedy (binary serialization)**](koute-speedy.md) | Fast custom-derive binary serializer, 3x-34x faster than bincode 1.3 to serialize, but unmaintained and dropped from the main benchmark. | A | 🟡 Holds up in part |
| [**borsh (Binary Object Representation Serializer for Hashing)**](near-borsh-rs.md) | Canonical binary format from NEAR for hashing and consensus; benchmarks at the same speed as bincode 1.3, not faster. | A | ❌ Doesn't hold up |
