---
id: fixture_java_ordered_list
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<ol><li>First</li><li>Second</li><li>Third</li></ol>", ConversionOptions.builder().build());
    }
}

```
