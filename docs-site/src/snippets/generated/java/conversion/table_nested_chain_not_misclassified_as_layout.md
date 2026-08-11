---
id: fixture_java_table_nested_chain_not_misclassified_as_layout
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<table><tr><td><table><tr><td><table><tr><td>leaf</td></tr></table></td></tr></table></td></tr></table>", ConversionOptions.builder().build());
    }
}

```
