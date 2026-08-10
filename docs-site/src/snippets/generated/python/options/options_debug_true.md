```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>Debug test</p>"
    options = ConversionOptions(debug=True)
    _ = convert(html, options)

main()

```
