---
id: fixture_java_code_inline_in_paragraph
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Call the <code>initialize()</code> method first.</p>", ConversionOptions.builder().build());
    }
}

```
