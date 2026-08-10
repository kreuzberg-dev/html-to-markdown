```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<p>Example code:</p><pre><code class="language-rust">fn main() { println!("Hello"); }</code></pre>'
    options = ConversionOptions(include_document_structure=True)
    _ = convert(html, options)

main()

```
