---
id: fixture_python_visitor_skip_links
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
            return "Skip"
    html = '<p>Before <a href="https://example.com">link text</a> after</p>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
