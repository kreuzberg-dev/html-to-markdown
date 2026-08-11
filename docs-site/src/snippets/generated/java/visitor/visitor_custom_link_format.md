---
id: fixture_java_visitor_custom_link_format
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Visit <a href=\"https://example.com\">Example</a> for more info.</p>", ConversionOptions.builder().build());
    }
}

```
