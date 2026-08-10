```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<ul><li><p>First paragraph in item.</p><p>Second paragraph in item.</p></li><li>Simple item</li></ul>"
    _ = convert(html, None)

main()

```
