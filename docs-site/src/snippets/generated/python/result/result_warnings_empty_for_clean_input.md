---
id: fixture_python_result_warnings_empty_for_clean_input
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<h1>Title</h1><p>Clean content with <a href='https://example.com'>a link</a>.</p>"
    _ = convert(html, None)

main()

```
