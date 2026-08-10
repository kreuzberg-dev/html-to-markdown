```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>"
    options = ConversionOptions(br_in_tables=True)
    _ = convert(html, options)

main()

```
