```python title="Python"
from html_to_markdown import convert, ConversionOptions, OutputFormat

def main() -> None:
    html = "<p>Simple paragraph.</p>"
    options = ConversionOptions(output_format=OutputFormat("Djot"))
    _ = convert(html, options)

main()

```
