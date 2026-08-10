```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>Before rule.</p><hr><p>After rule.</p>"
    _ = convert(html, None)

main()

```
