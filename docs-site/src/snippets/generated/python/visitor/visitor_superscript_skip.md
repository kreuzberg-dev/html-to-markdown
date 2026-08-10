```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_superscript(self, ctx, text):  # noqa: A002, ANN001, ANN202, ARG002
            return "Skip"
    html = "<p>The equation x<sup>3</sup> + y<sup>3</sup> = z<sup>3</sup> has no solutions.</p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
