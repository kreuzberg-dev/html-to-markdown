---
id: fixture_java_hidden_content_noscript_element_dropped
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>visible</p><noscript><p>secret noscript text</p></noscript><p>also visible</p>", ConversionOptions.builder().build());
    }
}

```
