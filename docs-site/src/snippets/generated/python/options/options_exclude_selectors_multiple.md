```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<body><nav class="nav">Menu</nav><p>Content</p><footer>Footer</footer></body>'
    options = ConversionOptions(exclude_selectors=[".nav", "footer"])
    _ = convert(html, options)

main()

```
