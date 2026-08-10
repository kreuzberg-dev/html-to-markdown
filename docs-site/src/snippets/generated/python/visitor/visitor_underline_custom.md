```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_underline(self, ctx, text):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": f'_{text}_'}
    html = "<p>This is <u>very important</u> text.</p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
