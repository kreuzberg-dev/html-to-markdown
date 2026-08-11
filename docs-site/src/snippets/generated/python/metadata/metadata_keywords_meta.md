---
id: fixture_python_metadata_keywords_meta
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<html><head><title>Page</title><meta name="keywords" content="rust, markdown, html, converter"></head><body><p>Content</p></body></html>'
    options = ConversionOptions(extract_metadata=True)
    _ = convert(html, options)

main()

```
