---
id: fixture_python_blockquote_text_then_paragraph_gets_blank_line
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<blockquote>Just text, then <p>a paragraph</p></blockquote>"
    _ = convert(html, None)

main()

```
