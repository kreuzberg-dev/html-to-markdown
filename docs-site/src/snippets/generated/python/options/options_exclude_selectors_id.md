```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<body><div id="ad-container">Buy stuff</div><p>Article text</p></body>'
    options = ConversionOptions(exclude_selectors=["#ad-container"])
    _ = convert(html, options)

main()

```
