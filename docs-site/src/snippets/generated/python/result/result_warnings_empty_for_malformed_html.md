---
id: fixture_python_result_warnings_empty_for_malformed_html
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>Unclosed paragraph<div>Mixed nesting</p></div>"
    _ = convert(html, None)

main()

```
