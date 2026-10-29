# Parsers and CSV

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Data formats](../README.md)</sub>

**The default:** The csv crate, nom, and hand-written str::split parsing.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**simd-csv (SIMD CSV reader)**](medialab-simd-csv.md) | CSV reader that pairs a state machine with SIMD byte searching; its author measures up to 4x-6x over the csv crate on long-cell data. | C | ✅ Holds up |
| [**winnow (parser combinators)**](winnow-rs-winnow.md) | Fork of nom that parsed canada.json 2.5x faster and built 3x faster than nom 8 in its author's comparison suite. | C | 🟡 Holds up in part |
