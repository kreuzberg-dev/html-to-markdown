---
id: fixture_java_semantic_details_summary
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<details><summary>Click to expand</summary><p>Hidden content here.</p></details>", ConversionOptions.builder().build());
    }
}

```
