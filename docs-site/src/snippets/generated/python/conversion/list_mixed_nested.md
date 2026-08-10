```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<ul><li>Item A<ol><li>Sub 1</li><li>Sub 2</li></ol></li><li>Item B</li></ul>"
    _ = convert(html, None)

main()

```
