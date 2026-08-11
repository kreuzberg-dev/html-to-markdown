---
id: fixture_python_options_exclude_selectors_vs_strip_tags
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<body><div class="wrapper"><p>Inner paragraph</p></div><p>Outer text</p></body>'
    options = ConversionOptions(exclude_selectors=[".wrapper"])
    _ = convert(html, options)

main()

```
