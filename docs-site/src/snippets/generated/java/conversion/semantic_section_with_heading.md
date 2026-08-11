---
id: fixture_java_semantic_section_with_heading
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<section><h3>Section Heading</h3><p>Section content.</p></section>", ConversionOptions.builder().build());
    }
}

```
