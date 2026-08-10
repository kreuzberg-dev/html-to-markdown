```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_heading(self, ctx, level, text, id):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": f'--- {text} ---'}
    html = "<h2>Section Title</h2><p>Content below heading.</p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
