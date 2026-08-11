---
id: fixture_java_options_keep_inline_images_in_paragraph
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
        var optionsJson = "{\"keep_inline_images_in\":[\"p\"]}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Text <img src='icon.png' alt='icon'> more text</p>", options);
    }
}

```
