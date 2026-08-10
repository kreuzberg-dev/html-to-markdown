```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_iframe(self, ctx, src):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": "[EMBEDDED: https://maps.example.com/embed]"}
    html = '<p>Embedded map:</p><iframe src="https://maps.example.com/embed" width="400" height="300"></iframe><p>End of map</p>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
