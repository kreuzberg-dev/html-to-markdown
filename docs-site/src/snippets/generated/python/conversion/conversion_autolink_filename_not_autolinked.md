```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<a href="foobar.png">foobar.png</a>'
    _ = convert(html, None)

main()

```
