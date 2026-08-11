---
id: fixture_python_visitor_input_custom
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
        def visit_input(self, ctx, input_type, name, value):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": f'[INPUT:{input_type}]'}
    html = '<form><label>Username: <input type="text" name="username" value=""></label><label>Password: <input type="password" name="password"></label></form>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
