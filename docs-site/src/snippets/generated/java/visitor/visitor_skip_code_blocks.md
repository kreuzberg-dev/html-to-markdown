---
id: fixture_java_visitor_skip_code_blocks
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Intro text</p><pre><code>let x = 42;</code></pre><p>Outro text</p>", ConversionOptions.builder().build());
    }
}

```
