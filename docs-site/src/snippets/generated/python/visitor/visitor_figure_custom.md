---
id: fixture_python_visitor_figure_custom
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
        def visit_figcaption(self, ctx, text):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": f'*{text}*'}
    html = '<article><h1>Article Title</h1><p>Introduction paragraph.</p><figure><img src="diagram.png" alt="System architecture diagram"><figcaption>Figure 1: System Architecture</figcaption></figure><p>Explanation of the figure.</p></article>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
