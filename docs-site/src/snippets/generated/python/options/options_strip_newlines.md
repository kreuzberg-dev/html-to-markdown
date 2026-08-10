```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>First paragraph.</p><p>Second paragraph.</p>"
    options = ConversionOptions(strip_newlines=True)
    _ = convert(html, options)

main()

```
