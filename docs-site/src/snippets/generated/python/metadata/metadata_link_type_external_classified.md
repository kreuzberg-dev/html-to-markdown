```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<p>See <a href="https://example.com">Example</a> for details.</p>'
    options = ConversionOptions(extract_metadata=True)
    _ = convert(html, options)

main()

```
