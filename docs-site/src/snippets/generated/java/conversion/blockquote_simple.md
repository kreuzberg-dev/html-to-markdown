---
id: fixture_java_blockquote_simple
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<blockquote><p>Quote text</p></blockquote>", ConversionOptions.builder().build());
    }
}

```
