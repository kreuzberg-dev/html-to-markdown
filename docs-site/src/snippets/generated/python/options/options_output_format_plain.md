```python title="Python"
from html_to_markdown import convert, ConversionOptions, OutputFormat

def main() -> None:
    html = "<h1>Title</h1><p>Some <strong>bold</strong> text.</p>"
    options = ConversionOptions(output_format=OutputFormat("Plain"))
    _ = convert(html, options)

main()

```
