```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<nav>Navigation</nav><p>Content</p><footer>Footer</footer>"
    options = ConversionOptions(preprocessing={"preset": "Minimal"})
    _ = convert(html, options)

main()

```
