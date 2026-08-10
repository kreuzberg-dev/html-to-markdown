```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_horizontal_rule(self, ctx, *args):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": "\n[DIVIDER]\n"}
    html = "<h1>Section A</h1><p>Content A</p><hr><h1>Section B</h1><p>Content B</p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
