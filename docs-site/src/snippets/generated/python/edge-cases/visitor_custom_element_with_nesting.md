---
id: fixture_python_visitor_custom_element_with_nesting
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
        def visit_custom_element(self, ctx, tag_name, html):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": "[CUSTOM WIDGET]"}
    html = '<div><custom-widget data-value="123"><p>Widget content here</p><span>With nested elements</span></custom-widget></div>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
