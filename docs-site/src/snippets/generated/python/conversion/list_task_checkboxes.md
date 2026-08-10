```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<ul><li><input type="checkbox" checked> Done task</li><li><input type="checkbox"> Pending task</li></ul>'
    _ = convert(html, None)

main()

```
