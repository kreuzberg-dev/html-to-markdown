```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<a href="https://example.com"><strong>Bold link</strong></a>'
    _ = convert(html, None)

main()

```
