```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<html lang="ar" dir="rtl"><head><title>RTL Document</title></head><body><p>This is right-to-left text.</p></body></html>'
    options = ConversionOptions(extract_metadata=True)
    _ = convert(html, options)

main()

```
