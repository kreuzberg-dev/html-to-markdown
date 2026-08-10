```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<h1>Title</h1><p>Clean content with <a href='https://example.com'>a link</a>.</p>"
    _ = convert(html, None)

main()

```
