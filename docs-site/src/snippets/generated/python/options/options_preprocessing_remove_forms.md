```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<p>Before</p><form><input type='text'/><button>Submit</button></form><p>After</p>"
    options = ConversionOptions(preprocessing={"remove_forms": True})
    _ = convert(html, options)

main()

```
