---
id: fixture_java_inline_code
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Use <code>console.log()</code> to debug</p>", ConversionOptions.builder().build());
    }
}

```
