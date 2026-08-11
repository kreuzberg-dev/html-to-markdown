---
id: fixture_python_options_escape_ascii_enabled
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>Text with # hash and [brackets] and * star</p>"
    options = ConversionOptions(escape_ascii=True)
    _ = convert(html, options)

main()

```
