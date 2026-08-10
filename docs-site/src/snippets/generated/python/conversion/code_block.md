```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<pre><code class=\"language-python\">print('hello')</code></pre>"
    _ = convert(html, None)

main()

```
