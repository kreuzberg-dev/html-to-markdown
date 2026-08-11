---
id: fixture_python_visitor_definition_list_skip
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
        def visit_definition_description(self, ctx, text):  # noqa: A002, ANN001, ANN202, ARG002
            return "Skip"
        def visit_definition_term(self, ctx, text):  # noqa: A002, ANN001, ANN202, ARG002
            return "Skip"
    html = "<p>Glossary:</p><dl><dt>Term A</dt><dd>Definition of term A</dd><dt>Term B</dt><dd>Definition of term B</dd></dl><p>End of glossary</p>"
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
