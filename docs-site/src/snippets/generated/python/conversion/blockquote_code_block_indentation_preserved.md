---
id: fixture_python_blockquote_code_block_indentation_preserved
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<blockquote><pre><code>line1\n    line2 indented</code></pre></blockquote>"
    _ = convert(html, None)

main()

```
