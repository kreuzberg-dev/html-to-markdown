---
id: fixture_python_options_exclude_selectors_class
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<body><div class="cookie-banner">Accept cookies</div><p>Main content</p></body>'
    options = ConversionOptions(exclude_selectors=[".cookie-banner"])
    _ = convert(html, options)

main()

```
