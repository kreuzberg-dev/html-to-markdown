---
id: fixture_python_visitor_skip_heading
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
            return "Skip"
    html = "<h1>Title</h1><p>Body text remains.</p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
