---
id: fixture_java_code_with_backticks_in_content
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Use <code>`backtick` here</code> carefully.</p>", ConversionOptions.builder().build());
    }
}

```
