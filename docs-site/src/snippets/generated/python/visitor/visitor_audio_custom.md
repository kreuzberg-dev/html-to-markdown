---
id: fixture_python_visitor_audio_custom
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert

def main() -> None:
    class _TestVisitor:
        def visit_audio(self, ctx, src):  # noqa: A002, ANN001, ANN202, ARG002
            return {"Custom": "[AUDIO: podcast.mp3]"}
    html = '<p>Listen to this: <audio src="podcast.mp3" controls></audio></p>'
    _ = convert(html, None, visitor=_TestVisitor())

main()

```
