```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>"
    _ = convert(html, None)

main()

```
