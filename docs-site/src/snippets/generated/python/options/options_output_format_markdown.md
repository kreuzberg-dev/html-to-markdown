---
id: fixture_python_options_output_format_markdown
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions, HeadingStyle, OutputFormat

def main() -> None:
    html = "<h1>Title</h1><p>Some text.</p>"
    options = ConversionOptions(heading_style=HeadingStyle("Atx"), output_format=OutputFormat("Markdown"))
    _ = convert(html, options)

main()

```
