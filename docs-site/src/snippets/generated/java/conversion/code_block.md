---
id: fixture_java_code_block
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<pre><code class=\"language-python\">print('hello')</code></pre>", ConversionOptions.builder().build());
    }
}

```
