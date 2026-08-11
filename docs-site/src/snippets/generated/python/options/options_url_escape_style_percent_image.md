---
id: fixture_python_options_url_escape_style_percent_image
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions, UrlEscapeStyle

def main() -> None:
    html = '<img src="/img (1) <draft>.png" alt="alt">'
    options = ConversionOptions(url_escape_style=UrlEscapeStyle("percent"))
    _ = convert(html, options)

main()

```
