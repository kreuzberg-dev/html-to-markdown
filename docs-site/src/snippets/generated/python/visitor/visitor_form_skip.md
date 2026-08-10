```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_form(self, ctx, action_url, method):  # noqa: A002, ANN001, ANN202, ARG002
            return "Skip"
    html = '<p>Before form</p><form><input type="email" name="email"></form><p>After form</p>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
