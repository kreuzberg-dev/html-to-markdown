---
id: fixture_python_visitor_form_custom
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
        def visit_form(self, ctx, action_url, method):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": "[FORM PLACEHOLDER]"}
    html = '<div><form action="/submit" method="POST"><label>Name: <input type="text" name="name"></label><button type="submit">Submit</button></form></div>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
