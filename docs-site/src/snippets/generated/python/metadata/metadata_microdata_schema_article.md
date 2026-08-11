---
id: fixture_python_metadata_microdata_schema_article
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<html><head><title>Article</title></head><body><article itemscope itemtype="https://schema.org/Article"><h1 itemprop="headline">Breaking News Today</h1><span itemprop="author">Jane Reporter</span><span itemprop="datePublished">2024-04-22</span><div itemprop="articleBody"><p>The article content goes here with important information about the breaking news story.</p></div></article></body></html>'
    options = ConversionOptions(extract_metadata=True)
    _ = convert(html, options)

main()

```
