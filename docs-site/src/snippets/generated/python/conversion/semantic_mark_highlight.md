```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>This is <mark>highlighted text</mark> in a sentence.</p>"
    _ = convert(html, None)

main()

```
