---
id: fixture_python_code_block
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<pre><code class=\"language-python\">print('hello')</code></pre>"
    _ = convert(html, None)

main()

```
