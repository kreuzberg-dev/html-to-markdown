```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_image(self, ctx, src, alt, title):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": f'[Image: {alt}]'}
    html = '<img src="banner.png" alt="Banner">'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
