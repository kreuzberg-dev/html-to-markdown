```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_subscript(self, ctx, text):  # noqa: A002, ANN001, ANN202, ARG002
            return "Skip"
    html = "<p>The formula C<sub>12</sub>H<sub>22</sub>O<sub>11</sub> is sugar.</p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
