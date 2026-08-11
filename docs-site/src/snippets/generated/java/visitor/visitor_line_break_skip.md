---
id: fixture_java_visitor_line_break_skip
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Address Line 1<br>Address Line 2<br>Address Line 3</p>", ConversionOptions.builder().build());
    }
}

```
