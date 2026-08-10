```python title="Python"
from html_to_markdown import convert, ConversionOptions, CodeBlockStyle

def main() -> None:
    html = "<pre><code>some code</code></pre>"
    options = ConversionOptions(code_block_style=CodeBlockStyle("Tildes"))
    _ = convert(html, options)

main()

```
