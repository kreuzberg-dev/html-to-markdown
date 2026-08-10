```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>The variable_name is defined.</p>"
    options = ConversionOptions(escape_underscores=True)
    _ = convert(html, options)

main()

```
