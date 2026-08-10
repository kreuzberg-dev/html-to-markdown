```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var optionsJson = "{\"include_document_structure\":true}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<table><tr><th>A</th></tr><tr><td>1</td></tr></table><p>Between</p><table><tr><th>B</th></tr><tr><td>2</td></tr></table>", options);
    }
}

```
