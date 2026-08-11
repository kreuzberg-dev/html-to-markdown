---
id: fixture_java_result_warnings_empty_for_malformed_html
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Unclosed paragraph<div>Mixed nesting</p></div>", ConversionOptions.builder().build());
    }
}

```
