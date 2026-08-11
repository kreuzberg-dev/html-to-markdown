---
id: fixture_java_visitor_continue_default
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Hello <strong>World</strong></p>", ConversionOptions.builder().build());
    }
}

```
