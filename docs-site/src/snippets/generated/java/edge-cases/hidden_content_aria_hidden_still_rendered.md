---
id: fixture_java_hidden_content_aria_hidden_still_rendered
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>visible</p><div aria-hidden=\"true\">still shown</div><p>also visible</p>", ConversionOptions.builder().build());
    }
}

```
