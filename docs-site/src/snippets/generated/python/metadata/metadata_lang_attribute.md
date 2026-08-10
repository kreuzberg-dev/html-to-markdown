```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<html lang="es"><head><title>Spanish Page</title></head><body><h1>Hola Mundo</h1><p>Este es un documento en español.</p></body></html>'
    options = ConversionOptions(extract_metadata=True)
    _ = convert(html, options)

main()

```
