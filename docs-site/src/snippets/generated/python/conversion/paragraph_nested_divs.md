```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<div><div><p>Nested text</p></div></div>"
    _ = convert(html, None)

main()

```
