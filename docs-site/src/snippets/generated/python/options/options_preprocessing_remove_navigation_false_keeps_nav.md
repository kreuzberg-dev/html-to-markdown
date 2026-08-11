---
id: fixture_python_options_preprocessing_remove_navigation_false_keeps_nav
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>"
    options = ConversionOptions(preprocessing={"remove_navigation": False})
    _ = convert(html, options)

main()

```
