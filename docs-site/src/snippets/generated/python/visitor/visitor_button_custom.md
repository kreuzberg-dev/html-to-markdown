---
id: fixture_python_visitor_button_custom
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
        def visit_button(self, ctx, text):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": f'[BTN:{text}]'}
    html = '<p>Confirm action: <button type="submit">Click me</button> or <button type="reset">Cancel</button></p>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
