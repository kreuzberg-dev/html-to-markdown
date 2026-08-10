```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>Copyright: &#169; Trade: &#174; Euro: &#8364; Hex: &#x00A9;</p>"
    _ = convert(html, None)

main()

```
