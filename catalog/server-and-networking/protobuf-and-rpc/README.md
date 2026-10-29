# Protocol Buffers and RPC

<sub>[fastrust](../../../README.md) › [Catalog](../../README.md) › [Server and networking](../README.md)</sub>

**The default:** prost and tonic.

| Entry | What it is | Tier | Verdict |
|---|---|:-:|---|
| [**buffa (protobuf with zero-copy view types)**](anthropics-buffa.md) | Pure-Rust protobuf with editions support; about even with prost on owned types, 1.1-2.8x faster decode through zero-copy views. | A | 🟡 Holds up in part |
| [**pajamax (synchronous gRPC server)**](wubingzheng-pajamax.md) | Thread-per-connection gRPC server with a gRPC-specific HTTP/2 parser; 3-30% faster than tonic in a third-party run, not the 10x its author cites. | A | 🟡 Holds up in part |
| [**wtx (gRPC server)**](c410-f3r-wtx.md) | Collection of web transports with its own HTTP/2 and gRPC stack; beat tonic in its author's grpc_bench run, lost by ~1.8x in the maintainer's. | A | 🟡 Holds up in part |
| [**quick-protobuf (borrowing protobuf decoder)**](tafia-quick-protobuf.md) | Protobuf crate whose generated types borrow strings and bytes from the input; 2018 numbers show faster reads than prost, slower writes. | C | 🟡 Holds up in part |
