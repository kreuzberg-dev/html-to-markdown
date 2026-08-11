---
id: fixture_python_visitor_deeply_nested_skip
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
    html = "<div><p>Outer <em>emphasis <strong>with bold <mark>and highlight</mark></strong></em> text</p></div>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
