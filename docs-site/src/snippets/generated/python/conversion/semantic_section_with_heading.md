---
id: fixture_python_semantic_section_with_heading
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<section><h3>Section Heading</h3><p>Section content.</p></section>"
    _ = convert(html, None)

main()

```
