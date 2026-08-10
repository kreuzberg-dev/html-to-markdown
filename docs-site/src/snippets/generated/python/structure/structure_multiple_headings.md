```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<h1>Main Title</h1><h2>Section One</h2><p>Section one content.</p><h2>Section Two</h2><p>Section two content.</p>"
    options = ConversionOptions(include_document_structure=True)
    _ = convert(html, options)

main()

```
