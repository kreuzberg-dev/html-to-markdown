```python
from html_to_markdown import ConversionOptions, convert

html = """
<html>
<head>
    <title>My Page</title>
    <meta name="description" content="An example page">
</head>
<body>
    <h1>Hello</h1>
</body>
</html>
"""

options = ConversionOptions(
    extract_metadata=True,
    extract_images=True,
)
result = convert(html, options)
markdown = result.content
metadata = result.metadata
print(metadata.document.title)
```
