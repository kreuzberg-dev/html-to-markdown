---
id: fixture_java_options_exclude_selectors_multiple
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
        var optionsJson = "{\"exclude_selectors\":[\".nav\",\"footer\"]}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<body><nav class=\"nav\">Menu</nav><p>Content</p><footer>Footer</footer></body>", options);
    }
}

```
