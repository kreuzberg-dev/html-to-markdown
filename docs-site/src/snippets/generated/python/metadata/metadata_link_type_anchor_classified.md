```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<p>Jump to <a href="#section">section</a> below.</p>'
    options = ConversionOptions(extract_metadata=True)
    _ = convert(html, options)

main()

```
