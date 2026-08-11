---
id: fixture_java_blockquote_text_then_paragraph_gets_blank_line
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<blockquote>Just text, then <p>a paragraph</p></blockquote>", ConversionOptions.builder().build());
    }
}

```
