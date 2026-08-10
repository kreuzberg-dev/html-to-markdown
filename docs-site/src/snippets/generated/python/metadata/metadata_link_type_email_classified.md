```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<p>Contact <a href="mailto:hello@example.com">us</a> directly.</p>'
    options = ConversionOptions(extract_metadata=True)
    _ = convert(html, options)

main()

```
