---
id: fixture_python_visitor_definition_list_custom
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_definition_term(self, ctx, text):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": f'**{text}**'}
    html = "<dl><dt>HTML</dt><dd>HyperText Markup Language</dd><dt>CSS</dt><dd>Cascading Style Sheets</dd></dl>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
