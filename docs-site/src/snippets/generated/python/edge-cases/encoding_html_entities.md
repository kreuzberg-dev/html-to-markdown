---
id: fixture_python_encoding_html_entities
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>&amp; &lt; &gt; &nbsp; &quot; &apos;</p>"
    _ = convert(html, None)

main()

```
