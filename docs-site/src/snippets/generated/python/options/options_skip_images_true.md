```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>Before <img src='test.jpg' alt='photo'> After</p>"
    options = ConversionOptions(skip_images=True)
    _ = convert(html, options)

main()

```
