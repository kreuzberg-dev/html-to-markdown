---
id: fixture_java_emphasis_strikethrough_s
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p><s>strikethrough</s></p>", ConversionOptions.builder().build());
    }
}

```
