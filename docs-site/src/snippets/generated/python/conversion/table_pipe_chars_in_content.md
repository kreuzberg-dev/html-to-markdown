```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<table><thead><tr><th>Expression</th><th>Result</th></tr></thead><tbody><tr><td>a | b</td><td>true</td></tr></tbody></table>"
    _ = convert(html, None)

main()

```
