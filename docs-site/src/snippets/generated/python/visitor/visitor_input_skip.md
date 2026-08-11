---
id: fixture_python_visitor_input_skip
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
            return "Skip"
    html = '<p>Sign up:</p><input type="text" name="email" placeholder="your@email.com"><input type="checkbox" name="agree"><p>Continue</p>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
