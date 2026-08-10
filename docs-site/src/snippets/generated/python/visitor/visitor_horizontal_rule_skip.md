```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_horizontal_rule(self, ctx, *args):  # noqa: A002, ANN001, ANN202, ARG002
            return "Skip"
    html = "<p>Part 1</p><hr><p>Part 2</p><hr><p>Part 3</p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
