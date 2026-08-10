```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var optionsJson = "{\"heading_style\":\"Atx\",\"output_format\":\"Markdown\"}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<h1>Title</h1><p>Some text.</p>", options);
    }
}

```
