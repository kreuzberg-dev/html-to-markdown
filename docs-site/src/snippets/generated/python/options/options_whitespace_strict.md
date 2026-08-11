---
id: fixture_python_options_whitespace_strict
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions, WhitespaceMode

def main() -> None:
    html = "<p>Preserved   spacing.</p>"
    options = ConversionOptions(whitespace_mode=WhitespaceMode("Strict"))
    _ = convert(html, options)

main()

```
