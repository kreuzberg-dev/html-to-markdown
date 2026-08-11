---
id: fixture_java_options_max_depth_default_unlimited
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<div><div><div><div><p>Deep content</p></div></div></div></div>", ConversionOptions.builder().build());
    }
}

```
