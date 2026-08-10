```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p><a href=\"https://example.com\" onclick=\"alert('xss')\">Click me</a></p><button onmouseover=\"steal_data()\">Hover me</button>"
    _ = convert(html, None)

main()

```
