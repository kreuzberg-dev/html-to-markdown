```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<figure><img src="sunset.jpg" alt="A sunset"><figcaption>Beautiful sunset over the ocean</figcaption></figure>'
    _ = convert(html, None)

main()

```
