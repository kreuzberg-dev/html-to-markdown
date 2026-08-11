---
id: fixture_python_options_url_escape_style_percent_link
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions, UrlEscapeStyle

def main() -> None:
    html = '<a href="/file (1).pdf">file</a>'
    options = ConversionOptions(url_escape_style=UrlEscapeStyle("percent"))
    _ = convert(html, options)

main()

```
