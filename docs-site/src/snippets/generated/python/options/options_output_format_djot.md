---
id: fixture_python_options_output_format_djot
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions, OutputFormat

def main() -> None:
    html = "<p>Simple paragraph.</p>"
    options = ConversionOptions(output_format=OutputFormat("Djot"))
    _ = convert(html, options)

main()

```
