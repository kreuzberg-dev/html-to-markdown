---
id: fixture_python_options_list_indent_width_four
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<ul><li>Outer<ul><li>Inner</li></ul></li></ul>"
    options = ConversionOptions(list_indent_width=4)
    _ = convert(html, options)

main()

```
