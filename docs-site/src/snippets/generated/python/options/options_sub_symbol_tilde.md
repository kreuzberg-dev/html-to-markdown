---
id: fixture_python_options_sub_symbol_tilde
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>H<sub>2</sub>O</p>"
    options = ConversionOptions(sub_symbol="~")
    _ = convert(html, options)

main()

```
