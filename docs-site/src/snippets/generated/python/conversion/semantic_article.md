```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<article><h2>Article Title</h2><p>Article body.</p></article>"
    _ = convert(html, None)

main()

```
