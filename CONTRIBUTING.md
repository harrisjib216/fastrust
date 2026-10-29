# Contributing

The catalog is only as good as its evidence, so the most useful contributions are corrections:

- **A newer benchmark** that changes a verdict or a tier, especially one run on a current rustc.
- **A default that caught up**: the standard library, Cargo or rustc shipped something that closes the gap.
- **A wrong number, a dead link, or a quote that no longer appears on its page.** `catalog verify` finds most of
  these.
- **A new entry** with published numbers against a default.

## Adding or changing an entry

1. Edit or add a record in `data/entries/`. The fields are described in [docs/schema.md](docs/schema.md).
2. Every number in `claimed_speedup` must appear in an `evidence` quote, copied exactly from the page at that quote's
   URL. Mark a clause you computed, such as a ratio, "(derived from page data)". `catalog check` enforces this.
3. Record when and how it was measured: the date, CPU, OS, rustc version and build flags (`--release`, LTO,
   `target-cpu`). A benchmark against a default that has since been replaced belongs in `caveats`.
4. Run the tools and commit the rendered output with your change:

   ```sh
   cargo run -p catalog -- check
   cargo run -p catalog -- verify
   cargo run -p catalog -- render
   ```

## What doesn't belong

- Crates with no performance claim. They may be good, but this catalog is about speed against a default.
- Benchmarks you ran yourself and haven't published. Publish them first (a gist or a repository with the code), then
  cite them.
