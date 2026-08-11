---
id: fixture_java_visitor_custom_blockquote
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<blockquote><p>A wise quote.</p></blockquote>", ConversionOptions.builder().build());
    }
}

```
