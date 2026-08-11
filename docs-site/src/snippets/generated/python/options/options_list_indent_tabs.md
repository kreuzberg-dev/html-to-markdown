---
id: fixture_python_options_list_indent_tabs
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions, ListIndentType

def main() -> None:
    html = "<ul><li>Parent<ul><li>Child</li></ul></li></ul>"
    options = ConversionOptions(list_indent_type=ListIndentType("Tabs"))
    _ = convert(html, options)

main()

```
