```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<ol><li>Step 1<ol><li>Step 1a</li><li>Step 1b</li></ol></li><li>Step 2</li></ol>"
    _ = convert(html, None)

main()

```
