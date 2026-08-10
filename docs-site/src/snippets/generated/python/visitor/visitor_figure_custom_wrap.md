```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_figure_end(self, ctx, output, *args):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": f'{output}\n[/FIGURE]\n'}
        def visit_figure_start(self, ctx, *args):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": "\n[FIGURE]\n"}
    html = '<section><h2>Gallery</h2><figure><img src="photo1.jpg" alt="Photo"><figcaption>Beautiful sunset</figcaption></figure></section>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
