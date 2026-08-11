---
id: fixture_python_metadata_link_type_external_classified
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<p>See <a href="https://example.com">Example</a> for details.</p>'
    options = ConversionOptions(extract_metadata=True)
    _ = convert(html, options)

main()

```
