---
id: fixture_python_options_wrap_disabled
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>This is a long paragraph that should not be wrapped at all because wrapping is disabled.</p>"
    options = ConversionOptions(wrap=False)
    _ = convert(html, options)

main()

```
