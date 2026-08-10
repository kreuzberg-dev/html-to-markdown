```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<p>The <abbr title="World Wide Web">WWW</abbr> is global.</p>'
    _ = convert(html, None)

main()

```
