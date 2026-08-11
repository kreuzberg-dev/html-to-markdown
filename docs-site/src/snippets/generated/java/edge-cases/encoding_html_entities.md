---
id: fixture_java_encoding_html_entities
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>&amp; &lt; &gt; &nbsp; &quot; &apos;</p>", ConversionOptions.builder().build());
    }
}

```
