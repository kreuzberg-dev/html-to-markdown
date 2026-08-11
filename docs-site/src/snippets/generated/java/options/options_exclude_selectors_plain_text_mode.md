---
id: fixture_java_options_exclude_selectors_plain_text_mode
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
        var optionsJson = "{\"exclude_selectors\":[\".nav\"],\"output_format\":\"Plain\"}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<body><div class=\"nav\">Navigation</div><p>Article body</p></body>", options);
    }
}

```
