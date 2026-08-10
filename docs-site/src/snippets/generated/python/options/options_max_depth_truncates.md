```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>"
    options = ConversionOptions(max_depth=3)
    _ = convert(html, options)

main()

```
