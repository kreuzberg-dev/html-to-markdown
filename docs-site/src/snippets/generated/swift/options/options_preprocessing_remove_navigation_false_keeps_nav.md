---
id: fixture_swift_options_preprocessing_remove_navigation_false_keeps_nav
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"preprocessing\":{\"remove_navigation\":false}}")
_ = try HtmlToMarkdown.convert(html: "<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>", options: _options)

```
