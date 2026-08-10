```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_strong(self, ctx, text):  # noqa: A002, ANN001, ANN202, ARG002
            return "Continue"
    html = "<p>Hello <strong>World</strong></p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
