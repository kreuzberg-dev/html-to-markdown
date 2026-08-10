```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_element_end(self, ctx, output, *args):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": "MODIFIED OUTPUT"}
    html = "<blockquote><p>Original quote</p></blockquote>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
