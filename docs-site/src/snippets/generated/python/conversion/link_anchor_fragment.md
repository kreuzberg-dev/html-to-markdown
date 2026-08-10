```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<a href="#section">Jump to section</a>'
    _ = convert(html, None)

main()

```
