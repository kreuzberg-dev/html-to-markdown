---
id: fixture_python_visitor_skip_images
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
        def visit_image(self, ctx, src, alt, title):  # noqa: A002, ANN001, ANN202, ARG002
            return "Skip"
    html = '<p>Before image</p><img src="photo.jpg" alt="A photo"><p>After image</p>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
