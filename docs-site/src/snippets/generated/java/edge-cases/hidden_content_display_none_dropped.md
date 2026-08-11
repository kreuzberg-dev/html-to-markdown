---
id: fixture_java_hidden_content_display_none_dropped
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>visible</p><div style=\"display:none\">secret hidden text</div><p>also visible</p>", ConversionOptions.builder().build());
    }
}

```
