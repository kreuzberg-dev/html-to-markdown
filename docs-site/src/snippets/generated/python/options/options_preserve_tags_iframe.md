```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>Before</p><iframe src='video.html' width='560'></iframe><p>After</p>"
    options = ConversionOptions(preserve_tags=["iframe"])
    _ = convert(html, options)

main()

```
