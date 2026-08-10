```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_button(self, ctx, text):  # noqa: A002, ANN001, ANN202, ARG002
            return "Skip"
    html = "<p>Actions available: <button>Save</button> <button>Delete</button> <button>Export</button></p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
