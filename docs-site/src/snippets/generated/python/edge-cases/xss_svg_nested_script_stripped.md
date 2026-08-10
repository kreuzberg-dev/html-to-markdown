```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>Before SVG.</p><svg xmlns=\"http://www.w3.org/2000/svg\"><script>alert('svg-xss')</script><text>SVG text</text></svg><p>After SVG.</p>"
    _ = convert(html, None)

main()

```
