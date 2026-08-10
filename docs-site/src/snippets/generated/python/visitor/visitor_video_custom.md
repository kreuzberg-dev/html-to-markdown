```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_video(self, ctx, src):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": f'[VIDEO: {src}]'}
    html = '<p>Watch our tutorial:</p><video src="tutorial.mp4" width="320" height="240" controls></video><p>Great content!</p>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
