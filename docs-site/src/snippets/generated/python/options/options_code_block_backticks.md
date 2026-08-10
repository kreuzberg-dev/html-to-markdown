```python title="Python"
from html_to_markdown import convert, ConversionOptions, CodeBlockStyle

def main() -> None:
    html = "<pre><code class=\"language-js\">console.log('hi');</code></pre>"
    options = ConversionOptions(code_block_style=CodeBlockStyle("Backticks"))
    _ = convert(html, options)

main()

```
