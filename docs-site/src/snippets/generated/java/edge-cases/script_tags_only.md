---
id: fixture_java_script_tags_only
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<html><head><script>alert('xss')</script></head><body><script>document.write('hello')</script></body></html>", ConversionOptions.builder().build());
    }
}

```
