```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<a href="mailto:user@example.com">Email us</a>'
    _ = convert(html, None)

main()

```
