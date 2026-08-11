---
id: fixture_java_options_list_indent_tabs
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
        var optionsJson = "{\"list_indent_type\":\"Tabs\"}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<ul><li>Parent<ul><li>Child</li></ul></li></ul>", options);
    }
}

```
