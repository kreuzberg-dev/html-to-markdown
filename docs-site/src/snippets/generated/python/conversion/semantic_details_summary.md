---
id: fixture_python_semantic_details_summary
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<details><summary>Click to expand</summary><p>Hidden content here.</p></details>"
    _ = convert(html, None)

main()

```
