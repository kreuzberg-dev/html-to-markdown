---
id: fixture_java_visitor_element_start_skip_entire_subtree
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<div><h1>Title</h1><p>Content</p></div>", ConversionOptions.builder().build());
    }
}

```
