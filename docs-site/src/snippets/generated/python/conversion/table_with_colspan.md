```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<table><thead><tr><th colspan="2">Full Name</th></tr></thead><tbody><tr><td>John</td><td>Doe</td></tr></tbody></table>'
    _ = convert(html, None)

main()

```
