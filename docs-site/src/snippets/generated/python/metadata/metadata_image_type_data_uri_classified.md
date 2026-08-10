```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<p><img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==" alt="pixel"></p>'
    options = ConversionOptions(extract_metadata=True)
    _ = convert(html, options)

main()

```
