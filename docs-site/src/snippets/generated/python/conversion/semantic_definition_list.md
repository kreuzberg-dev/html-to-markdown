```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<dl><dt>HTML</dt><dd>HyperText Markup Language</dd><dt>CSS</dt><dd>Cascading Style Sheets</dd></dl>"
    _ = convert(html, None)

main()

```
