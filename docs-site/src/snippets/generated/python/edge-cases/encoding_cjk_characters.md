```python title="Python"
from html_to_markdown import convert

def main() -> None:
    html = "<p>中文内容</p><p>日本語テキスト</p><p>한국어 텍스트</p>"
    _ = convert(html, None)

main()

```
