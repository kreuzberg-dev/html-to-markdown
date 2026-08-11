---
id: fixture_java_paragraph_multiple
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>First paragraph.</p><p>Second paragraph.</p>", ConversionOptions.builder().build());
    }
}

```
