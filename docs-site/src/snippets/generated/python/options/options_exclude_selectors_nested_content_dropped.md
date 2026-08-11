---
id: fixture_python_options_exclude_selectors_nested_content_dropped
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = '<body><aside class="sidebar"><h2>Related</h2><p>Sidebar text</p></aside><main><p>Main text</p></main></body>'
    options = ConversionOptions(exclude_selectors=[".sidebar"])
    _ = convert(html, options)

main()

```
