```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_line_break(self, ctx, *args):  # noqa: A002, ANN001, ANN202, ARG002
            return "Skip"
    html = "<p>Address Line 1<br>Address Line 2<br>Address Line 3</p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
