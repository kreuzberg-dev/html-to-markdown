```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<body><aside class="sidebar"><h2>Related</h2><p>Sidebar text</p></aside><main><p>Main text</p></main></body>'
    options = ConversionOptions(exclude_selectors=[".sidebar"])
    _ = convert(html, options)

main()

```
