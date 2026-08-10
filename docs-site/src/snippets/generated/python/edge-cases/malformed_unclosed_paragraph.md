```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>This paragraph is never closed"
    _ = convert(html, None)

main()

```
