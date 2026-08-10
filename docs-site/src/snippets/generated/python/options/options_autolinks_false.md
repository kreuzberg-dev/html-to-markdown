```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p><a href='https://example.com'>https://example.com</a></p>"
    options = ConversionOptions(autolinks=False)
    _ = convert(html, options)

main()

```
