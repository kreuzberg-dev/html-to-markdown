```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<nav>NavSection</nav><p>Paragraph</p>"
    options = ConversionOptions(preprocessing={"enabled": False})
    _ = convert(html, options)

main()

```
