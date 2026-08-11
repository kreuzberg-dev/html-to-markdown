---
id: fixture_python_visitor_mark_skip
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
        def visit_mark(self, ctx, text):  # noqa: A002, ANN001, ANN202, ARG002
            return "Skip"
    html = "<p>Key insight: <mark>always validate input</mark> for security.</p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
