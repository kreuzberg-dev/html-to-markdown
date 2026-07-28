---
name: extracting-metadata
description: Use when extracting metadata from HTML — title, description, language, Open Graph, JSON-LD / Microdata / RDFa, headers, links, and images. Covers the --json output shape and the --extract-metadata flag.
---

# Extracting metadata

Use this when the user wants structured metadata out of HTML rather than (or in
addition to) the Markdown body — page title, description, language, Open Graph
tags, structured data, the heading outline, links, or image references.

Metadata lives in `result.metadata` and is surfaced on the CLI through
`--json`. Metadata is extracted by default; all sub-fields below populate
automatically inside `result.metadata` whenever `--json` is used. There are no
per-field extraction flags.

## Get all metadata

```bash
html-to-markdown --json input.html | jq '.metadata'

# Extraction-only (skip the Markdown body)
html-to-markdown --json --no-content input.html | jq '.metadata'
```

## Metadata sub-fields

```json
{
  "metadata": {
    "document": { "title": "...", "description": "...", "language": "en", "open_graph": {"title": "..."} },
    "headers": [ { "level": 1, "text": "Main Heading" } ],
    "links":   [ { "href": "https://example.com", "link_type": "external" } ],
    "images":  [ { "src": "photo.jpg", "alt": "A photo", "image_type": "external" } ],
    "structured_data": [ /* JSON-LD, Microdata, RDFa blocks */ ]
  }
}
```

## Metadata flag

There is one metadata flag. The sub-fields above (`document`, `headers`,
`links`, `images`, `structured_data`) are always populated under
`result.metadata` when `--json` is set — select what you need with `jq`.

| Flag | Effect |
| ---- | ------ |
| `--extract-metadata` | (plain-text mode, no `--json`) emit title + meta tags as an HTML-comment header at the top of the Markdown output |

```bash
# Pull just the document-level metadata and the heading outline
html-to-markdown --json --no-content input.html \
  | jq '{title: .metadata.document.title, lang: .metadata.document.language, outline: [.metadata.headers[].text]}'
```

## Common queries

```bash
# Title + canonical language
html-to-markdown --json input.html | jq '{title: .metadata.document.title, lang: .metadata.document.language}'

# External links only
html-to-markdown --json input.html | jq '[.metadata.links[] | select(.link_type == "external") | .href]'

# Open Graph card
html-to-markdown --json input.html | jq '.metadata.document.open_graph'

# JSON-LD blocks
html-to-markdown --json input.html | jq '.metadata.structured_data'
```

## Programmatic access

```python
from html_to_markdown import convert

result = convert(html)
meta = result.metadata
print(meta.document.title)        # "My Article"
print(meta.document.language)     # "en"
print(meta.headers[0].text)       # "Main Heading"
print(meta.links[0].link_type)    # "external"
print(meta.images[0].alt)         # "A photo"
```

Metadata is available from the single `convert()` call in Python, Rust, Go,
Ruby, and Elixir; in TypeScript read it off the returned result object. In Rust
it requires the `metadata` feature (on by default).

See `../html-to-markdown/references/cli-reference.md` (Metadata section) and
`../html-to-markdown/references/configuration.md` for field details.
