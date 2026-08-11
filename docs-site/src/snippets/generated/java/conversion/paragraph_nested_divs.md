---
id: fixture_java_paragraph_nested_divs
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<div><div><p>Nested text</p></div></div>", ConversionOptions.builder().build());
    }
}

```
