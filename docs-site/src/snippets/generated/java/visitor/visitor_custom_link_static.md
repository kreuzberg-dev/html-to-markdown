---
id: fixture_java_visitor_custom_link_static
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<a href=\"https://example.com\">Click here</a>", ConversionOptions.builder().build());
    }
}

```
