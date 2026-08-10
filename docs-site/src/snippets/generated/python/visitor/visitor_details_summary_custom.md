```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_summary(self, ctx, text):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": f'[EXPANDABLE] {text}'}
    html = "<details><summary>Click to expand</summary><p>This content is initially hidden.</p><p>But can be revealed by the user.</p></details>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
