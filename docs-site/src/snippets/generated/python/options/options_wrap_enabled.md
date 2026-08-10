```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>This is a long paragraph that should be wrapped at the specified column width when the wrap option is enabled.</p>"
    options = ConversionOptions(wrap=True, wrap_width=40)
    _ = convert(html, options)

main()

```
