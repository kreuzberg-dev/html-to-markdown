---
id: fixture_python_code_block_no_language
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<pre><code>plain code here</code></pre>"
    _ = convert(html, None)

main()

```
