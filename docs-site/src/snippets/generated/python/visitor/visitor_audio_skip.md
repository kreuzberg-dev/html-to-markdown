```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_audio(self, ctx, src):  # noqa: A002, ANN001, ANN202, ARG002
            return "Skip"
    html = '<p>Background music:</p><audio src="music.ogg" autoplay></audio><p>Enjoy!</p>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
