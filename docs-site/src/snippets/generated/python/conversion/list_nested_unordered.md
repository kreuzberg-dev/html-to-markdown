```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<ul><li>Parent A<ul><li>Child A1</li><li>Child A2</li></ul></li><li>Parent B</li></ul>"
    _ = convert(html, None)

main()

```
