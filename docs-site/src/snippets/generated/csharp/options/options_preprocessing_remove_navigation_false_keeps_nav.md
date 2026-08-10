```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>", new ConversionOptions { Preprocessing = new PreprocessingOptions { RemoveNavigation = false } });

```
