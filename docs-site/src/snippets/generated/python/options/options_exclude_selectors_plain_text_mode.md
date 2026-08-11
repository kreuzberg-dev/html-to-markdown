---
id: fixture_python_options_exclude_selectors_plain_text_mode
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions, OutputFormat

def main() -> None:
    html = '<body><div class="nav">Navigation</div><p>Article body</p></body>'
    options = ConversionOptions(exclude_selectors=[".nav"], output_format=OutputFormat("Plain"))
    _ = convert(html, options)

main()

```
