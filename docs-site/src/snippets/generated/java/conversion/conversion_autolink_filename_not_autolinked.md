---
id: fixture_java_conversion_autolink_filename_not_autolinked
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<a href=\"foobar.png\">foobar.png</a>", ConversionOptions.builder().build());
    }
}

```
