```python title="Python"
from html_to_markdown import convert, ConversionOptions, CodeBlockStyle

def main() -> None:
    html = "<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>"
    options = ConversionOptions(code_block_style=CodeBlockStyle("Backticks"))
    _ = convert(html, options)

main()

```
