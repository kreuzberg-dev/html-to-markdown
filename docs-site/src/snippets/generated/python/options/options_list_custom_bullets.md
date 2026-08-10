```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<ul><li>Item A</li><li>Item B</li></ul>"
    options = ConversionOptions(bullets="*")
    _ = convert(html, options)

main()

```
