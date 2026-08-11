---
id: fixture_java_blockquote_code_block_indentation_preserved
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<blockquote><pre><code>line1\n    line2 indented</code></pre></blockquote>", ConversionOptions.builder().build());
    }
}

```
