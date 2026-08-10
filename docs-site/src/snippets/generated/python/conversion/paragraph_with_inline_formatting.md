```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<p>This has <strong>bold</strong>, <em>italic</em>, and a <a href="https://example.com">link</a>.</p>'
    _ = convert(html, None)

main()

```
