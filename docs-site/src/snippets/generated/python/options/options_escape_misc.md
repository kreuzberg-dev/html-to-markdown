```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>Use # and | and ~ in text.</p>"
    options = ConversionOptions(escape_misc=True)
    _ = convert(html, options)

main()

```
