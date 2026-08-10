```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<a href="/docs/intro.html">/docs/intro.html</a>'
    _ = convert(html, None)

main()

```
