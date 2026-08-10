```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_video(self, ctx, src):  # noqa: A002, ANN001, ANN202, ARG002
            return "Skip"
    html = '<h2>Demo</h2><video src="demo.webm"></video><p>See the demo above.</p>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
