```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p><a href='https://example.com'>Link</a></p>"
    options = ConversionOptions(default_title=True)
    _ = convert(html, options)

main()

```
