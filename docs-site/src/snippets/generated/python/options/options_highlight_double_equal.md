```python title="Python"
from html_to_markdown import convert, ConversionOptions, HighlightStyle

def main() -> None:
    html = "<p>Text with <mark>highlighted</mark> here.</p>"
    options = ConversionOptions(highlight_style=HighlightStyle("DoubleEqual"))
    _ = convert(html, options)

main()

```
