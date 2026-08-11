---
id: fixture_java_malformed_overlapping_tags
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p><b><i>bold and italic</b></i></p>", ConversionOptions.builder().build());
    }
}

```
