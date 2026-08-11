---
id: fixture_java_options_preprocessing_remove_navigation_false_keeps_nav
language: java
target: java
level: typecheck
requires: []
side_effect: safe
---

```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var optionsJson = "{\"preprocessing\":{\"remove_navigation\":false}}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>", options);
    }
}

```
