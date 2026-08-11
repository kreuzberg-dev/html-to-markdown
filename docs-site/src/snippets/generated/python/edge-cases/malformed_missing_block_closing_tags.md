---
id: fixture_python_malformed_missing_block_closing_tags
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<div><h1>Title<p>First paragraph<p>Second paragraph</div>"
    _ = convert(html, None)

main()

```
