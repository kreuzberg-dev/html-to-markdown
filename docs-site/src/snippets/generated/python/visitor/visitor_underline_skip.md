---
id: fixture_python_visitor_underline_skip
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
        def visit_underline(self, ctx, text):  # noqa: A002, ANN001, ANN202, ARG002
            return "Skip"
    html = "<p>Normal text with <u>underlined part</u> and more text.</p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
