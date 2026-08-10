```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_link(self, ctx, href, text, title):  # noqa: A002, ANN001, ANN202, ARG002
            return f'[{text}](https://new-cdn.com/file.pdf)'
    html = '<a href="https://old-cdn.com/file.pdf">Download</a>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
