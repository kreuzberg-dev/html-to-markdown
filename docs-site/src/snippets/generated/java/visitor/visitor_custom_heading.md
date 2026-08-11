---
id: fixture_java_visitor_custom_heading
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<h2>Section Title</h2><p>Content below heading.</p>", ConversionOptions.builder().build());
    }
}

```
