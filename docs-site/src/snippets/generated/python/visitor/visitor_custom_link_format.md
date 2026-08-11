---
id: fixture_python_visitor_custom_link_format
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
        def visit_link(self, ctx, href, text, title):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": f'{text} ({href})'}
    html = '<p>Visit <a href="https://example.com">Example</a> for more info.</p>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
