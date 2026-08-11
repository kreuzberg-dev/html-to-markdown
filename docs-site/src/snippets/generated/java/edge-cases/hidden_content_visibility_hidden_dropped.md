---
id: fixture_java_hidden_content_visibility_hidden_dropped
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>visible</p><span style=\"visibility:hidden\">secret hidden span</span><p>also visible</p>", ConversionOptions.builder().build());
    }
}

```
