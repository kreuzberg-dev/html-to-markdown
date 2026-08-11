---
id: fixture_csharp_options_preprocessing_remove_navigation_false_keeps_nav
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>", new ConversionOptions { Preprocessing = new PreprocessingOptions { RemoveNavigation = false } });

```
