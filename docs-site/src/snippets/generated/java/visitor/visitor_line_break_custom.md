---
id: fixture_java_visitor_line_break_custom
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>First line<br>Second line<br>Third line</p>", ConversionOptions.builder().build());
    }
}

```
