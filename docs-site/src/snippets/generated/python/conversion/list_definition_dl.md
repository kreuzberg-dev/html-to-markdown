```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<dl><dt>Term One</dt><dd>Definition of term one.</dd><dt>Term Two</dt><dd>Definition of term two.</dd></dl>"
    _ = convert(html, None)

main()

```
