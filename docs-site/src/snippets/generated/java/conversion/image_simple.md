---
id: fixture_java_image_simple
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<img src=\"photo.jpg\" alt=\"A photo\">", ConversionOptions.builder().build());
    }
}

```
