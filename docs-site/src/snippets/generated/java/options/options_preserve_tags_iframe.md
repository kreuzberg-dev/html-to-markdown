---
id: fixture_java_options_preserve_tags_iframe
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
        var optionsJson = "{\"preserve_tags\":[\"iframe\"]}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Before</p><iframe src='video.html' width='560'></iframe><p>After</p>", options);
    }
}

```
