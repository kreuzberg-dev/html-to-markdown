---
id: fixture_java_semantic_hr
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Above</p><hr><p>Below</p>", ConversionOptions.builder().build());
    }
}

```
