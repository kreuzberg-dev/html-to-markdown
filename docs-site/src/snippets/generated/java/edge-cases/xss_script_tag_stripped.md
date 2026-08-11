---
id: fixture_java_xss_script_tag_stripped
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>", ConversionOptions.builder().build());
    }
}

```
