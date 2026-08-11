---
id: fixture_python_options_debug_true
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>Debug test</p>"
    options = ConversionOptions(debug=True)
    _ = convert(html, options)

main()

```
