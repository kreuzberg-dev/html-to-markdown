```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<blockquote><p>Quote intro:</p><ul><li>Point one</li><li>Point two</li></ul></blockquote>"
    _ = convert(html, None)

main()

```
