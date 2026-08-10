```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>Hello world</p>"
    options = ConversionOptions(exclude_selectors=[])
    _ = convert(html, options)

main()

```
