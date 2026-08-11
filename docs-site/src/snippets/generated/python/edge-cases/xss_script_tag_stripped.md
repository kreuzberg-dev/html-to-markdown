---
id: fixture_python_xss_script_tag_stripped
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>"
    _ = convert(html, None)

main()

```
