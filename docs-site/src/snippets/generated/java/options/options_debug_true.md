```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var optionsJson = "{\"debug\":true}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Debug test</p>", options);
    }
}

```
