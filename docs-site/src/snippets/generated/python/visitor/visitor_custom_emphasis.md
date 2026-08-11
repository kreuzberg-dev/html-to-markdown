---
id: fixture_python_visitor_custom_emphasis
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
        def visit_emphasis(self, ctx, text):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": f'>>>{text}<<<'}
    html = "<p>This is <em>important</em> text.</p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
