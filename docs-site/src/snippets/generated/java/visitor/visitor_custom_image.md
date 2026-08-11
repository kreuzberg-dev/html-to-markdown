---
id: fixture_java_visitor_custom_image
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<img src=\"banner.png\" alt=\"Banner\">", ConversionOptions.builder().build());
    }
}

```
