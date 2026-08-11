---
id: fixture_python_form_select_options
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<form><label>Color:</label><select><option value="red">Red</option><option value="blue" selected>Blue</option><option value="green">Green</option></select></form>'
    options = ConversionOptions(preprocessing={"remove_forms": False})
    _ = convert(html, options)

main()

```
