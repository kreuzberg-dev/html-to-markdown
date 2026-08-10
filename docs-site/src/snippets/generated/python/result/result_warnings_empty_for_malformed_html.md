```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>Unclosed paragraph<div>Mixed nesting</p></div>"
    _ = convert(html, None)

main()

```
