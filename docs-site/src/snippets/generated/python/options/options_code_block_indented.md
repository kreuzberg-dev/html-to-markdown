---
id: fixture_python_options_code_block_indented
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions, CodeBlockStyle

def main() -> None:
    html = "<pre><code>print('hello')</code></pre>"
    options = ConversionOptions(code_block_style=CodeBlockStyle("Indented"))
    _ = convert(html, options)

main()

```
