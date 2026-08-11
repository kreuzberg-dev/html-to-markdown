---
id: fixture_python_options_newline_spaces
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions, NewlineStyle

def main() -> None:
    html = "<p>First<br>Second</p>"
    options = ConversionOptions(newline_style=NewlineStyle("Spaces"))
    _ = convert(html, options)

main()

```
