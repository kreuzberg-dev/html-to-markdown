---
id: fixture_python_options_link_style_reference
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions, LinkStyle

def main() -> None:
    html = "<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>"
    options = ConversionOptions(link_style=LinkStyle("Reference"))
    _ = convert(html, options)

main()

```
