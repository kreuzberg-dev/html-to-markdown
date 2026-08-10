```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<blockquote><p>First paragraph.</p><p>Second paragraph.</p></blockquote>"
    _ = convert(html, None)

main()

```
