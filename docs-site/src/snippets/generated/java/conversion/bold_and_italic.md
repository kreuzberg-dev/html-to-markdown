---
id: fixture_java_bold_and_italic
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p><strong><em>both</em></strong></p>", ConversionOptions.builder().build());
    }
}

```
