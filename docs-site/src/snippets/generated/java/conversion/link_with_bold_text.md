---
id: fixture_java_link_with_bold_text
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<a href=\"https://example.com\"><strong>Bold link</strong></a>", ConversionOptions.builder().build());
    }
}

```
