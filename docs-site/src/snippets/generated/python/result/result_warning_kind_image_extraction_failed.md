```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<p>Text<img src="data:BADMIME" alt="broken">end</p>'
    options = ConversionOptions(extract_images=True)
    _ = convert(html, options)

main()

```
