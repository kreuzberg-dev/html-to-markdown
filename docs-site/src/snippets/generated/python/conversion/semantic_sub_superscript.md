```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>H<sub>2</sub>O and E=mc<sup>2</sup></p>"
    _ = convert(html, None)

main()

```
