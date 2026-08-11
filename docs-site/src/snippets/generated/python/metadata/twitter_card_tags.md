---
id: fixture_python_twitter_card_tags
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = '<html><head><meta name="twitter:card" content="summary_large_image"><meta name="twitter:site" content="@examplesite"><meta name="twitter:title" content="Twitter Card Title"><meta name="twitter:description" content="Twitter card description."><meta name="twitter:image" content="https://example.com/twitter-image.jpg"></head><body><p>Content</p></body></html>'
    _ = convert(html, None)

main()

```
