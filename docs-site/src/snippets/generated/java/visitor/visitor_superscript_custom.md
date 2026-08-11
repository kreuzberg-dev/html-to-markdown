---
id: fixture_java_visitor_superscript_custom
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Einstein's E=mc<sup>2</sup> revolutionized physics.</p>", ConversionOptions.builder().build());
    }
}

```
