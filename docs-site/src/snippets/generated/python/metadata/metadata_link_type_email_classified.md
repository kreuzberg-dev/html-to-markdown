---
id: fixture_python_metadata_link_type_email_classified
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<p>Contact <a href="mailto:hello@example.com">us</a> directly.</p>'
    options = ConversionOptions(extract_metadata=True)
    _ = convert(html, options)

main()

```
