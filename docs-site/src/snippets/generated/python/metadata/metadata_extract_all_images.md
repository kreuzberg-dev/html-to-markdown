```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<html><head><title>Gallery</title></head><body><img src="https://example.com/photo1.jpg" alt="Photo 1"><img src="https://example.com/photo2.png" alt="Photo 2"><img src="/local/image.webp" alt="Local image"></body></html>'
    options = ConversionOptions(extract_metadata=True)
    _ = convert(html, options)

main()

```
