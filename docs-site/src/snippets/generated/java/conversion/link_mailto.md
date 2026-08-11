---
id: fixture_java_link_mailto
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<a href=\"mailto:user@example.com\">Email us</a>", ConversionOptions.builder().build());
    }
}

```
