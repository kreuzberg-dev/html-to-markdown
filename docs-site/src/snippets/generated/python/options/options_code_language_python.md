```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<pre><code>def hello(): pass</code></pre>"
    options = ConversionOptions(code_language="python")
    _ = convert(html, options)

main()

```
