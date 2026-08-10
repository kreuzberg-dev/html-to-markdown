```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<html><head><script>alert('xss')</script></head><body><script>document.write('hello')</script></body></html>"
    _ = convert(html, None)

main()

```
