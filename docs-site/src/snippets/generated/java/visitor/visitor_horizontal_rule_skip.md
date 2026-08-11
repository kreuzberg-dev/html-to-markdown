---
id: fixture_java_visitor_horizontal_rule_skip
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Part 1</p><hr><p>Part 2</p><hr><p>Part 3</p>", ConversionOptions.builder().build());
    }
}

```
