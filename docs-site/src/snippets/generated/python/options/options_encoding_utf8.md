```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>Café naïve résumé</p>"
    options = ConversionOptions(encoding="utf-8")
    _ = convert(html, options)

main()

```
