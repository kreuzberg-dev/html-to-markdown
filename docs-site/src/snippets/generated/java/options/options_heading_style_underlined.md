```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var optionsJson = "{\"heading_style\":\"Underlined\"}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<h1>Main Title</h1>", options);
    }
}

```
