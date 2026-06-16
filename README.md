# mdbook-tagger

An mdbook preprocessor that generates tag index pages from frontmatter metadata.

## Features

- Scans markdown files for `tags` in YAML frontmatter
- Generates `tags/` directory with per-tag index pages
- Works as both **mdbook preprocessor** and **standalone CLI**

## Frontmatter Format

```yaml
---
tags: [rust, wasm, tutorial]
---
```

Or multiline:

```yaml
---
tags:
  - rust
  - wasm
  - tutorial
---
```

## Usage

### Standalone CLI

```bash
# Generate tag pages for a book
mdbook-tagger generate ./my-book
```

### As mdbook Preprocessor

Add to `book.toml`:

```toml
[preprocessor.tagger]
command = "mdbook-tagger preprocess"
```

Then run `mdbook build` as usual.

## Generated Output

For each unique tag, a page is created at `tags/<tag>.md` listing all articles with that tag. A `tags/SUMMARY.md` index page is also generated.

## Build

```bash
cargo build --release
```

## License

MIT
