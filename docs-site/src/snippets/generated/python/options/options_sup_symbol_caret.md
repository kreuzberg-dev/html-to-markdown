---
id: fixture_python_options_sup_symbol_caret
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>x<sup>2</sup></p>"
    options = ConversionOptions(sup_symbol="^")
    _ = convert(html, options)

main()

```
