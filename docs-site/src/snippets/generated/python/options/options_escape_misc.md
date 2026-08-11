---
id: fixture_python_options_escape_misc
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>Use # and | and ~ in text.</p>"
    options = ConversionOptions(escape_misc=True)
    _ = convert(html, options)

main()

```
