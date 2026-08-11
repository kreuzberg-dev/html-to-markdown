```python
from html_to_markdown import convert
from html_to_markdown.exceptions import InvalidInputError

# Binary data (detected via magic bytes) is rejected before parsing.
html = "%PDF-1.4 not actually HTML"

try:
    result = convert(html)
    markdown = result.content
except InvalidInputError as error:
    print(f"invalid input: {error}")
```
