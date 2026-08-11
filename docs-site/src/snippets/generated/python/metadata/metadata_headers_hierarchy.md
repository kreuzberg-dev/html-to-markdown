---
id: fixture_python_metadata_headers_hierarchy
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<html><head><title>Docs</title></head><body><h1>Introduction</h1><h2>Getting Started</h2><h3>Installation</h3><h3>Configuration</h3><h2>Advanced Usage</h2><h3>Custom Options</h3></body></html>"
    options = ConversionOptions(extract_metadata=True)
    _ = convert(html, options)

main()

```
