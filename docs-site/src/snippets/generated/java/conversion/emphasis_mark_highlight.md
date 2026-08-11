---
id: fixture_java_emphasis_mark_highlight
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p><mark>highlighted</mark></p>", ConversionOptions.builder().build());
    }
}

```
