---
id: fixture_python_options_max_depth_zero_empty
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>Hello</p>"
    options = ConversionOptions(max_depth=0)
    _ = convert(html, options)

main()

```
