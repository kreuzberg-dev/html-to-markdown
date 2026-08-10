```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<form><label>Message:</label><textarea>Default text content</textarea></form>"
    options = ConversionOptions(preprocessing={"remove_forms": False})
    _ = convert(html, options)

main()

```
