---
id: fixture_java_options_br_in_tables_false
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
        var optionsJson = "{\"br_in_tables\":false}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<table><tr><th>Col</th></tr><tr><td>A<br>B</td></tr></table>", options);
    }
}

```
