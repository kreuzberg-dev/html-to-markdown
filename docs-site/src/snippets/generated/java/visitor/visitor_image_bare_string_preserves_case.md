---
id: fixture_java_visitor_image_bare_string_preserves_case
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<img src=\"PhotoOne.JPG\" alt=\"Sunset Over Bay\">", ConversionOptions.builder().build());
    }
}

```
