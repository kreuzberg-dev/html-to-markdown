---
id: fixture_java_visitor_subscript_custom
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>H<sub>2</sub>O is water.</p>", ConversionOptions.builder().build());
    }
}

```
