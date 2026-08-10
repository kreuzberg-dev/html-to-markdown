```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_definition_description(self, ctx, text):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": f'> {text}'}
        def visit_definition_term(self, ctx, text):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": f'### {text}'}
    html = "<dl><dt>Python</dt><dd>A high-level programming language</dd><dt>JavaScript</dt><dd>A scripting language for web browsers</dd></dl>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
