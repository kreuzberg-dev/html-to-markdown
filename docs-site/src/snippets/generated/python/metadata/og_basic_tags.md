---
id: fixture_python_og_basic_tags
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<html><head><title>Fallback Title</title><meta property="og:title" content="OG Title"><meta property="og:description" content="OG description text."><meta property="og:image" content="https://example.com/image.jpg"></head><body><p>Content</p></body></html>'
    _ = convert(html, None)

main()

```
