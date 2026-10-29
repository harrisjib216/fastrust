# Lazy statics and one-time init

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Concurrency and runtime](../README.md)</sub>

**The default:** lazy_static.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**once_cell (sync::Lazy)**](matklad-once-cell.md) | Lazy statics without a macro, now mostly in std as OnceLock and LazyLock; access speed matches lazy_static, so pick it for API, not speed. | B | ❌ Doesn't hold up |
