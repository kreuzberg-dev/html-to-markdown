```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_custom_element(self, ctx, tag_name, html):  # noqa: A002, ANN001, ANN202, ARG002
            return "PreserveHtml"
    html = "<article><p>Article text</p><x-custom>Custom element with content</x-custom><p>More article text</p></article>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
