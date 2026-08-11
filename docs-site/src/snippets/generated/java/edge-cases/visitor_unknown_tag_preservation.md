---
id: fixture_java_visitor_unknown_tag_preservation
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<article><p>Article text</p><x-custom>Custom element with content</x-custom><p>More article text</p></article>", ConversionOptions.builder().build());
    }
}

```
