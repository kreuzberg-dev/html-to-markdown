---
id: fixture_python_options_code_block_tildes
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions, CodeBlockStyle

def main() -> None:
    html = "<pre><code>let x = 1;</code></pre>"
    options = ConversionOptions(code_block_style=CodeBlockStyle("Tildes"))
    _ = convert(html, options)

main()

```
