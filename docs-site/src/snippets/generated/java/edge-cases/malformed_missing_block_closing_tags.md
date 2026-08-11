---
id: fixture_java_malformed_missing_block_closing_tags
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<div><h1>Title<p>First paragraph<p>Second paragraph</div>", ConversionOptions.builder().build());
    }
}

```
