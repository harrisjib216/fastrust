# Maps and sets

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Collections](../README.md)</sub>

**The default:** std HashMap, HashSet and BTreeMap.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**std HashMap and HashSet (hashbrown SwissTable, Rust 1.36)**](std-collections-hashmap.md) | Since Rust 1.36, std's HashMap is the hashbrown SwissTable, about 2x faster on lookups than the old Robin Hood table with the same hasher. | C | ✅ Holds up |
| [**hashbrown crate (SwissTable with foldhash as default hasher)**](rust-lang-hashbrown.md) | The SwissTable crate behind std's HashMap; on today's toolchains its speed edge over std is only its foldhash default hasher. | A | 🟡 Holds up in part |
| [**indexset (two-level order-statistic B-tree)**](brurucy-indexset.md) | Ordered set and map built from fixed-capacity Vec leaves; faster scans and rank lookups than std's BTreeMap, slower point lookups and removals. | A | 🟡 Holds up in part |
