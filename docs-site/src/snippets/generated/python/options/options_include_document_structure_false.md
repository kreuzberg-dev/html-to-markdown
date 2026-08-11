---
id: fixture_python_options_include_document_structure_false
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<article><h1>Heading</h1><p>Paragraph body.</p></article>"
    options = ConversionOptions(include_document_structure=False)
    _ = convert(html, options)

main()

```
