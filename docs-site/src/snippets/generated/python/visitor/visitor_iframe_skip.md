```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_iframe(self, ctx, src):  # noqa: A002, ANN001, ANN202, ARG002
            return "Skip"
    html = '<h3>Reviews</h3><iframe src="https://widget.example.com/reviews"></iframe><p>See reviews from our partners.</p>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
