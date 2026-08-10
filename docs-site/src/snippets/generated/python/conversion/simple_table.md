```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<table><thead><tr><th>Name</th><th>Age</th></tr></thead><tbody><tr><td>Alice</td><td>30</td></tr></tbody></table>"
    _ = convert(html, None)

main()

```
