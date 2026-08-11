---
id: fixture_python_visitor_figure_skip
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
        def visit_figure_start(self, ctx, *args):  # noqa: A002, ANN001, ANN202, ARG002
            return "Skip"
    html = '<p>See the chart below:</p><figure><img src="chart.svg"><figcaption>Revenue Trends 2020-2024</figcaption></figure><p>As shown in the chart above.</p>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
