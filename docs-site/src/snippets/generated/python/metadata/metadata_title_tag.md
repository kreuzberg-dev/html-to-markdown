```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<html><head><title>My Page</title></head><body><p>Content</p></body></html>"
    options = ConversionOptions(extract_metadata=True)
    _ = convert(html, options)

main()

```
