```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>Use <code>`backtick` here</code> carefully.</p>"
    _ = convert(html, None)

main()

```
