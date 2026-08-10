```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<div><h1>Title<p>First paragraph<p>Second paragraph</div>"
    _ = convert(html, None)

main()

```
