---
id: fixture_java_options_br_in_tables_true
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
        var optionsJson = "{\"br_in_tables\":true}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>", options);
    }
}

```
