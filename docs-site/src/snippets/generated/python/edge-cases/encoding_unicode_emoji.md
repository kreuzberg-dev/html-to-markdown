---
id: fixture_python_encoding_unicode_emoji
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>Hello 🌍 World 🚀</p><p>Stars: ⭐ ✨</p>"
    _ = convert(html, None)

main()

```
