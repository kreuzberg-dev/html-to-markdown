```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>Hello</p>"
    options = ConversionOptions(max_depth=0)
    _ = convert(html, options)

main()

```
