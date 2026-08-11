---
id: fixture_java_options_wrap_enabled
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
        var optionsJson = "{\"wrap\":true,\"wrap_width\":40}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>This is a long paragraph that should be wrapped at the specified column width when the wrap option is enabled.</p>", options);
    }
}

```
