---
id: fixture_python_metadata_microdata_schema_organization
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<html><head><title>Company</title></head><body><div itemscope itemtype="https://schema.org/Organization"><span itemprop="name">Acme Corp</span><span itemprop="foundingDate">2020</span><span itemprop="url">https://acmecorp.example.com</span><span itemprop="logo">https://acmecorp.example.com/logo.png</span></div></body></html>'
    options = ConversionOptions(extract_metadata=True)
    _ = convert(html, options)

main()

```
