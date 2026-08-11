---
id: fixture_python_options_extract_metadata_true
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<html><head><title>Test Page</title><meta name='description' content='A test page'></head><body><p>Content</p></body></html>"
    options = ConversionOptions(extract_metadata=True)
    _ = convert(html, options)

main()

```
