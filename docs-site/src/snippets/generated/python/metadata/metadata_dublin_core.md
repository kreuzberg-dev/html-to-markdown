---
id: fixture_python_metadata_dublin_core
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<html><head><title>Scholarly Work</title><meta name="DC.title" content="Principles of Knowledge Management"><meta name="DC.creator" content="Dr. Alice Johnson"><meta name="DC.date" content="2023-06-15"><meta name="DC.subject" content="Knowledge Management"><meta name="DC.publisher" content="Academic Press"></head><body><p>This is a scholarly article.</p></body></html>'
    options = ConversionOptions(extract_metadata=True)
    _ = convert(html, options)

main()

```
