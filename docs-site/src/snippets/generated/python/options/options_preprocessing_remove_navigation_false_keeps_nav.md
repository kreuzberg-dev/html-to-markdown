```python title="Python"
from html_to_markdown import convert, ConversionOptions

def main() -> None:
    html = "<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>"
    options = ConversionOptions(preprocessing={"remove_navigation": False})
    _ = convert(html, options)

main()

```
