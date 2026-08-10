```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_subscript(self, ctx, text):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": f'~{text}~'}
    html = "<p>H<sub>2</sub>O is water.</p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
