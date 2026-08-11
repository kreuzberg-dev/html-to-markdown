---
id: fixture_python_options_whitespace_normalized
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions, WhitespaceMode

def main() -> None:
    html = "<p>Text   with    extra   spaces.</p>"
    options = ConversionOptions(whitespace_mode=WhitespaceMode("Normalized"))
    _ = convert(html, options)

main()

```
