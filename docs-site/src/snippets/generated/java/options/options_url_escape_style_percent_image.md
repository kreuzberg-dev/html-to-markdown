---
id: fixture_java_options_url_escape_style_percent_image
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
        var optionsJson = "{\"url_escape_style\":\"percent\"}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<img src=\"/img (1) <draft>.png\" alt=\"alt\">", options);
    }
}

```
