```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<h1>One</h1>\n<!-- /// --->\n<p>Two</p>"
    _ = convert(html, None)

main()

```
