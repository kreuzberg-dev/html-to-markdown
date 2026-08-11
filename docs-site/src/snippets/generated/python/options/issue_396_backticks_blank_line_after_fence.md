---
id: fixture_python_issue_396_backticks_blank_line_after_fence
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions, CodeBlockStyle

def main() -> None:
    html = "<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>"
    options = ConversionOptions(code_block_style=CodeBlockStyle("Backticks"))
    _ = convert(html, options)

main()

```
