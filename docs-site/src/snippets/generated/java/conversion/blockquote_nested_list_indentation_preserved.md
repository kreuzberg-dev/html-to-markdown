---
id: fixture_java_blockquote_nested_list_indentation_preserved
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<blockquote><ul><li>item a<ul><li>sub a1</li></ul></li></ul></blockquote>", ConversionOptions.builder().build());
    }
}

```
