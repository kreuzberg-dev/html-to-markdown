---
id: fixture_python_structure_sibling_h1_groups
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<h1>Chapter One</h1><h2>Section A</h2><p>Section A content.</p><h1>Chapter Two</h1><h2>Section B</h2><p>Section B content.</p>"
    options = ConversionOptions(include_document_structure=True)
    _ = convert(html, options)

main()

```
