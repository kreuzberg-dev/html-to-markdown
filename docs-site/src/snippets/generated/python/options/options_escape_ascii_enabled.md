```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>Text with # hash and [brackets] and * star</p>"
    options = ConversionOptions(escape_ascii=True)
    _ = convert(html, options)

main()

```
