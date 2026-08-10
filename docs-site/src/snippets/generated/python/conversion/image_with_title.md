```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<img src="chart.png" alt="Sales chart" title="Q3 Sales">'
    _ = convert(html, None)

main()

```
