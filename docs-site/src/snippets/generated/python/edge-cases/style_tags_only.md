```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<html><head><style>body { color: red; }</style></head><body><style>.foo { margin: 0; }</style></body></html>"
    _ = convert(html, None)

main()

```
