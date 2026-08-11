---
id: fixture_java_result_tables_without_structure_flag
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>", ConversionOptions.builder().build());
    }
}

```
