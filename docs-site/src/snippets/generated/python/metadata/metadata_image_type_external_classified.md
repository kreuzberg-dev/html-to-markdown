---
id: fixture_python_metadata_image_type_external_classified
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<p><img src="https://example.com/photo.jpg" alt="A photo"></p>'
    options = ConversionOptions(extract_metadata=True)
    _ = convert(html, options)

main()

```
