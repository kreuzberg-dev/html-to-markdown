---
id: fixture_python_visitor_element_start_skip_entire_subtree
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
        def visit_element_start(self, ctx, *args):  # noqa: A002, ANN001, ANN202, ARG002
            return "Skip"
    html = "<div><h1>Title</h1><p>Content</p></div>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
