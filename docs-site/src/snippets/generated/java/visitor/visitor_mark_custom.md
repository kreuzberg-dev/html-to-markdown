---
id: fixture_java_visitor_mark_custom
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>This is a <mark>highlighted passage</mark> in the text.</p>", ConversionOptions.builder().build());
    }
}

```
