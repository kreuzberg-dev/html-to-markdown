```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>Use 2*3 = 6 in math.</p>"
    options = ConversionOptions(escape_asterisks=True)
    _ = convert(html, options)

main()

```
