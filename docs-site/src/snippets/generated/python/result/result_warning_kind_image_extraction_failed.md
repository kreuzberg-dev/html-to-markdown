---
id: fixture_python_result_warning_kind_image_extraction_failed
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<p>Text<img src="data:BADMIME" alt="broken">end</p>'
    options = ConversionOptions(extract_images=True)
    _ = convert(html, options)

main()

```
