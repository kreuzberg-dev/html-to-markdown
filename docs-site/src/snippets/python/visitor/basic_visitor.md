```python
from html_to_markdown import ConversionOptions, convert

html = '<p>Visit <a href="https://example.com">our site</a>.</p>'


class CustomVisitor:
    def visit_link(self, ctx, href, text, title):
        return {"type": "continue"}

    def visit_image(self, ctx, src, alt, title):
        return {"type": "continue"}


options = ConversionOptions(visitor=CustomVisitor())
result = convert(html, options)
markdown = result.content
print(markdown)
```
