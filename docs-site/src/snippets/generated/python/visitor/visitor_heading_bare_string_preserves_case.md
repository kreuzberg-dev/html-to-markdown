---
id: fixture_python_visitor_heading_bare_string_preserves_case
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_heading(self, ctx, level, text, id):  # noqa: A002, ANN001, ANN202, ARG002
            return f'## {text} ##'
    html = "<h2>Important Section Title</h2><p>Body.</p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
