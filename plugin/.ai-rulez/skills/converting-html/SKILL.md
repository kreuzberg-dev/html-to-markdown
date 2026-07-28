---
name: converting-html
description: Use when converting HTML to Markdown, Djot, or plain text. Covers output formats, heading and code-block styles, lists, escaping, wrapping, and HTML preprocessing.
---

# Converting HTML

Use this when the user wants to turn HTML into clean Markdown (or Djot) — a
saved page, an HTML email body, a fragment, or a string. The CLI takes
**flags only** (no subcommands); `FILE` is positional and stdin is the default
input when no file is given.

## Basic conversion

```bash
# File to stdout
html-to-markdown input.html

# File to a file
html-to-markdown input.html -o output.md

# From stdin
cat page.html | html-to-markdown
echo '<h1>Title</h1><p>Body</p>' | html-to-markdown
```

The default (no `--json`) prints plain Markdown text. Use `--json` only when you
need structured metadata/tables (see the `extracting-metadata` and
`extracting-tables` skills).

## Output format

```bash
html-to-markdown input.html --output-format markdown   # default
html-to-markdown input.html --output-format djot        # Djot markup
html-to-markdown input.html -f djot                      # short form
```

To get *plain text* (no Markdown syntax), strip tags and treat blocks inline:

```bash
html-to-markdown input.html --convert-as-inline --strip-tags "script,style"
```

## Heading and code-block styles

| Flag | Values | Default | Effect |
| ---- | ------ | ------- | ------ |
| `--heading-style` | `atx`, `underlined`, `atx-closed` | `atx` | `# h1` vs `h1\n===` vs `# h1 #` |
| `--code-block-style` | `backticks`, `indented`, `tildes` | `backticks` | Fence style for code blocks |
| `--code-language` / `-l` | string | `""` | Default language for fenced blocks |

```bash
html-to-markdown input.html --heading-style underlined --code-block-style tildes
```

## Lists and text formatting

```bash
# Bullet characters cycle by nesting depth; indent width 1–8
html-to-markdown input.html --bullets '*+-' --list-indent-width 2

# Indent with tabs instead of spaces
html-to-markdown input.html --list-indent-type tabs

# Emphasis symbol and escaping
html-to-markdown input.html --strong-em-symbol _ --escape-misc
```

Escaping flags (`--escape-asterisks`, `--escape-underscores`, `--escape-misc`,
`--escape-ascii`) trade readability for strict CommonMark safety. Use
`--escape-ascii` only when the output must round-trip through a strict parser.

## Wrapping

```bash
html-to-markdown input.html --wrap --wrap-width 100   # off by default; 80 if --wrap with no width
```

## Preprocessing (clean noisy pages)

Strip navigation, ads, and forms before converting — essential for scraped web
pages:

```bash
html-to-markdown input.html --preprocess                     # standard preset
html-to-markdown input.html --preprocess --preset aggressive  # strip more
html-to-markdown input.html --preprocess --keep-navigation    # keep <nav>
html-to-markdown input.html --preprocess --keep-forms         # keep <form>
```

Presets: `minimal`, `standard` (default), `aggressive`. `--preset`,
`--keep-navigation`, and `--keep-forms` all require `--preprocess`.

## Programmatic equivalents

```python
from html_to_markdown import convert, ConversionOptions, PreprocessingOptions

result = convert(
    html,
    ConversionOptions(heading_style="atx", code_block_style="backticks", wrap=True, wrap_width=100),
    PreprocessingOptions(enabled=True, preset="aggressive"),
)
print(result.content)
```

```rust
use html_to_markdown_rs::{convert, ConversionOptions, HeadingStyle};

let options = ConversionOptions::builder().heading_style(HeadingStyle::Atx).wrap(true).build();
let result = convert(html, Some(options))?;
```

See `../html-to-markdown/references/cli-reference.md` for the full flag set and
`../html-to-markdown/references/configuration.md` for every option default.
