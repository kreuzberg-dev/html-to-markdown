---
id: fixture_java_result_warnings_empty_for_clean_input
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<h1>Title</h1><p>Clean content with <a href='https://example.com'>a link</a>.</p>", ConversionOptions.builder().build());
    }
}

```
