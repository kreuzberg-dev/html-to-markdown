---
id: fixture_java_issue_396_backticks_blank_line_after_fence
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
        var optionsJson = "{\"code_block_style\":\"Backticks\"}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>", options);
    }
}

```
