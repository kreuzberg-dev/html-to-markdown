---
id: fixture_python_visitor_custom_link_static
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
        def visit_link(self, ctx, href, text, title):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": "[REDACTED LINK]"}
    html = '<a href="https://example.com">Click here</a>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
