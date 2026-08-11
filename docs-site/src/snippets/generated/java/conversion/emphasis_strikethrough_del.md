---
id: fixture_java_emphasis_strikethrough_del
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p><del>deleted text</del></p>", ConversionOptions.builder().build());
    }
}

```
