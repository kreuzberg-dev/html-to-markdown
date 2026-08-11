---
id: fixture_java_options_max_depth_truncates
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
        var optionsJson = "{\"max_depth\":3}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>", options);
    }
}

```
