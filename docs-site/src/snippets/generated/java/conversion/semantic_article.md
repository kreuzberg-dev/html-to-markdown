---
id: fixture_java_semantic_article
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<article><h2>Article Title</h2><p>Article body.</p></article>", ConversionOptions.builder().build());
    }
}

```
