---
id: fixture_python_form_input_elements
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<form><label for="name">Name:</label><input type="text" id="name" placeholder="Enter name"></form>'
    options = ConversionOptions(preprocessing={"remove_forms": False})
    _ = convert(html, options)

main()

```
