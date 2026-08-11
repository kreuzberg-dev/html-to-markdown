---
id: fixture_java_visitor_mark_skip
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Key insight: <mark>always validate input</mark> for security.</p>", ConversionOptions.builder().build());
    }
}

```
