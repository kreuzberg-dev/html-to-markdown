```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<h1>Top Level</h1><p>Top intro.</p><h2>Mid Level</h2><p>Mid content.</p><h3>Deep Level</h3><p>Deep content.</p>"
    options = ConversionOptions(include_document_structure=True)
    _ = convert(html, options)

main()

```
