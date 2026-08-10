```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<table><thead><tr><th>Name</th><th>Score</th></tr></thead><tbody><tr><td>Alice</td><td>100</td></tr><tr><td>Bob</td><td>42</td></tr></tbody></table>"
    options = ConversionOptions(compact_tables=False)
    _ = convert(html, options)

main()

```
