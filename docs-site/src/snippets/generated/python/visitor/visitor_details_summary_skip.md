---
id: fixture_python_visitor_details_summary_skip
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
        def visit_details(self, ctx, is_open):  # noqa: A002, ANN001, ANN202, ARG002
            return "Skip"
    html = "<p>Main content here.</p><details><summary>Hidden section</summary><p>Secret details</p></details><p>More main content.</p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
