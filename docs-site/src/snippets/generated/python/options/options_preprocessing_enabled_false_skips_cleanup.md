---
id: fixture_python_options_preprocessing_enabled_false_skips_cleanup
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<nav>NavSection</nav><p>Paragraph</p>"
    options = ConversionOptions(preprocessing={"enabled": False})
    _ = convert(html, options)

main()

```
