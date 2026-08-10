```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<a href="mailto:a@b.com">a@b.com</a>'
    _ = convert(html, None)

main()

```
