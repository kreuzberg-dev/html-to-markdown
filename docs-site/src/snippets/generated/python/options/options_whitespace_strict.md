```python title="Python"
from html_to_markdown import convert, ConversionOptions, WhitespaceMode

def main() -> None:
    html = "<p>Preserved   spacing.</p>"
    options = ConversionOptions(whitespace_mode=WhitespaceMode("Strict"))
    _ = convert(html, options)

main()

```
