---
id: fixture_python_blockquote_nested_list_indentation_preserved
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<blockquote><ul><li>item a<ul><li>sub a1</li></ul></li></ul></blockquote>"
    _ = convert(html, None)

main()

```
