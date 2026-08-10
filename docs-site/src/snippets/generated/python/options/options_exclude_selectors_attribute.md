```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<body><div role="complementary">Sidebar</div><p>Primary text</p></body>'
    options = ConversionOptions(exclude_selectors=["[role='complementary']"])
    _ = convert(html, options)

main()

```
