---
id: fixture_python_options_code_block_backticks
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions, CodeBlockStyle

def main() -> None:
    html = "<pre><code class=\"language-js\">console.log('hi');</code></pre>"
    options = ConversionOptions(code_block_style=CodeBlockStyle("Backticks"))
    _ = convert(html, options)

main()

```
