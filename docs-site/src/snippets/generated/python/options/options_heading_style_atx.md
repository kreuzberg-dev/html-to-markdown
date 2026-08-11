---
id: fixture_python_options_heading_style_atx
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions, HeadingStyle

def main() -> None:
    html = "<h1>Title</h1><h2>Subtitle</h2>"
    options = ConversionOptions(heading_style=HeadingStyle("Atx"))
    _ = convert(html, options)

main()

```
