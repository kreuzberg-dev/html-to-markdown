---
id: fixture_java_options_link_style_reference
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
        var optionsJson = "{\"link_style\":\"Reference\"}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>", options);
    }
}

```
