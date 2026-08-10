```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<html><head><title>Navigation</title></head><body><nav itemscope itemtype="https://schema.org/BreadcrumbList"><span itemprop="itemListElement" itemscope itemtype="https://schema.org/ListItem"><a itemprop="item" href="https://example.com"><span itemprop="name">Home</span></a></span><span itemprop="itemListElement" itemscope itemtype="https://schema.org/ListItem"><a itemprop="item" href="https://example.com/products"><span itemprop="name">Products</span></a></span><span itemprop="itemListElement" itemscope itemtype="https://schema.org/ListItem"><span itemprop="name">Current Page</span></span></nav></body></html>'
    options = ConversionOptions(extract_metadata=True, preprocessing={"remove_navigation": False})
    _ = convert(html, options)

main()

```
