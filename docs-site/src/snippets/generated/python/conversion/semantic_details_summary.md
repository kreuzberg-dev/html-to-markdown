```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<details><summary>Click to expand</summary><p>Hidden content here.</p></details>"
    _ = convert(html, None)

main()

```
