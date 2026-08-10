```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>Items:</p><ul><li>Alpha</li><li>Beta</li><li>Gamma</li></ul>"
    options = ConversionOptions(include_document_structure=True)
    _ = convert(html, options)

main()

```
