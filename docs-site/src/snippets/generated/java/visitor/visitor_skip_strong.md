---
id: fixture_java_visitor_skip_strong
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Normal <strong>bold text</strong> normal</p>", ConversionOptions.builder().build());
    }
}

```
