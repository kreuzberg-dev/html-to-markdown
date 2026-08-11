---
id: fixture_python_table_nested_chain_not_misclassified_as_layout
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<table><tr><td><table><tr><td><table><tr><td>leaf</td></tr></table></td></tr></table></td></tr></table>"
    _ = convert(html, None)

main()

```
