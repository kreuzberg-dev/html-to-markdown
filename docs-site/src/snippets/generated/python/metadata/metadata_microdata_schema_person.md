---
id: fixture_python_metadata_microdata_schema_person
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<html><head><title>Contact</title></head><body><div itemscope itemtype="https://schema.org/Person"><span itemprop="name">John Smith</span><span itemprop="email">john@example.com</span><span itemprop="telephone">+1-555-0100</span></div></body></html>'
    options = ConversionOptions(extract_metadata=True)
    _ = convert(html, options)

main()

```
