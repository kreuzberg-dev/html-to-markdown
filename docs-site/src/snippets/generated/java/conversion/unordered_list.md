---
id: fixture_java_unordered_list
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>", ConversionOptions.builder().build());
    }
}

```
