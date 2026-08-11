---
id: fixture_java_semantic_sub_superscript
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>H<sub>2</sub>O and E=mc<sup>2</sup></p>", ConversionOptions.builder().build());
    }
}

```
