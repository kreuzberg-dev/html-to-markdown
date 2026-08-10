```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<h1>Chapter One</h1><p>Chapter intro.</p><h2>Section One</h2><p>Section content.</p>"
    options = ConversionOptions(include_document_structure=True)
    _ = convert(html, options)

main()

```
