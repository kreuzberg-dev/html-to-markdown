```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<h1>Title</h1><p>A paragraph of text.</p>"
    options = ConversionOptions(include_document_structure=True)
    _ = convert(html, options)

main()

```
