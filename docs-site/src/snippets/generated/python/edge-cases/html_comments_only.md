```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<!-- This is a comment --><!-- Another comment -->"
    _ = convert(html, None)

main()

```
