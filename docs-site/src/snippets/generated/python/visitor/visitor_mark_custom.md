```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_mark(self, ctx, text):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": f'=={text}=='}
    html = "<p>This is a <mark>highlighted passage</mark> in the text.</p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
