---
id: fixture_python_options_strong_em_underscore
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p><strong>bold</strong> and <em>italic</em></p>"
    options = ConversionOptions(strong_em_symbol="_")
    _ = convert(html, options)

main()

```
