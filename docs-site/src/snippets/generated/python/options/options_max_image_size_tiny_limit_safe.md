```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<p>Tiny limit: <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==" alt="pixel"></p>'
    options = ConversionOptions(extract_images=True, max_image_size=10)
    _ = convert(html, options)

main()

```
