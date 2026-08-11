---
id: fixture_python_malformed_bogus_comment_triple_dash
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<h1>One</h1>\n<!-- /// --->\n<p>Two</p>"
    _ = convert(html, None)

main()

```
