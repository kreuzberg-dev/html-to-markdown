```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<nav>Menu</nav><article><h1>Title</h1><p>Content</p></article><aside>Sidebar</aside><footer>Footer</footer>"
    options = ConversionOptions(preprocessing={"preset": "Aggressive"})
    _ = convert(html, options)

main()

```
