```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<div><div><div><div><p>Deep content</p></div></div></div></div>"
    _ = convert(html, None)

main()

```
