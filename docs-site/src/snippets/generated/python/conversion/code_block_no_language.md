```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<pre><code>plain code here</code></pre>"
    _ = convert(html, None)

main()

```
