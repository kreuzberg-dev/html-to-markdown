---
id: fixture_python_hidden_content_noscript_element_dropped
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>visible</p><noscript><p>secret noscript text</p></noscript><p>also visible</p>"
    _ = convert(html, None)

main()

```
