```python title="Python"
from html_to_markdown import convert, ConversionOptions, NewlineStyle

def main() -> None:
    html = "<p>Line one<br>Line two</p>"
    options = ConversionOptions(newline_style=NewlineStyle("Backslash"))
    _ = convert(html, options)

main()

```
