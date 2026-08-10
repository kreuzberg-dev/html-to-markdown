```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_code_block(self, ctx, lang, code):  # noqa: A002, ANN001, ANN202, ARG002
            return "Skip"
    html = "<p>Intro text</p><pre><code>let x = 42;</code></pre><p>Outro text</p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
