---
id: fixture_java_semantic_abbr
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>The <abbr title=\"World Wide Web\">WWW</abbr> is global.</p>", ConversionOptions.builder().build());
    }
}

```
