```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>Text <img src='icon.png' alt='icon'> more text</p>"
    options = ConversionOptions(keep_inline_images_in=["p"])
    _ = convert(html, options)

main()

```
