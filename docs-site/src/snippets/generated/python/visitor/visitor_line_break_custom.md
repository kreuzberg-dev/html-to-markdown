---
id: fixture_python_visitor_line_break_custom
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
        def visit_line_break(self, ctx, *args):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": " | "}
    html = "<p>First line<br>Second line<br>Third line</p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
