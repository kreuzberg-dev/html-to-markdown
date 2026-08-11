---
id: fixture_java_line_break_br_tag
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>First line.<br>Second line.</p>", ConversionOptions.builder().build());
    }
}

```
