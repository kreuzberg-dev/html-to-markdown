```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_blockquote(self, ctx, content, depth):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": f'QUOTE: "{content}"'}
    html = "<blockquote><p>A wise quote.</p></blockquote>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
