```python title="Python"
from html_to_markdown import convert, ConversionOptions, HighlightStyle

def main() -> None:
    html = "<p>Text with <mark>plain</mark> content.</p>"
    options = ConversionOptions(highlight_style=HighlightStyle("None"))
    _ = convert(html, options)

main()

```
