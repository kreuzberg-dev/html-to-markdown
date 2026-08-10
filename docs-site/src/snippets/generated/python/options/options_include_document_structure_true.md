```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<article><h1>Heading</h1><p>Paragraph body.</p></article>"
    options = ConversionOptions(include_document_structure=True)
    _ = convert(html, options)

main()

```
