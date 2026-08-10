```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<html lang="en" dir="ltr"><head><title>LTR Document</title></head><body><p>This is left-to-right text.</p></body></html>'
    options = ConversionOptions(extract_metadata=True)
    _ = convert(html, options)

main()

```
