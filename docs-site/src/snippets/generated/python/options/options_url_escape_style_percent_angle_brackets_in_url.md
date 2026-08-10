```python title="Python"
from html_to_markdown import convert, ConversionOptions, UrlEscapeStyle

def main() -> None:
    html = '<a href="/file (1) <draft>.pdf">file</a>'
    options = ConversionOptions(url_escape_style=UrlEscapeStyle("percent"))
    _ = convert(html, options)

main()

```
