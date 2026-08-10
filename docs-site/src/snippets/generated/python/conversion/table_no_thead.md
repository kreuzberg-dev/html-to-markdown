```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<table><tr><td>Product</td><td>Price</td></tr><tr><td>Apple</td><td>1.00</td></tr></table>"
    _ = convert(html, None)

main()

```
