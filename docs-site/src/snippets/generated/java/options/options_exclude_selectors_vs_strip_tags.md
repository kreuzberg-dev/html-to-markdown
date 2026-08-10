```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var optionsJson = "{\"exclude_selectors\":[\".wrapper\"]}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<body><div class=\"wrapper\"><p>Inner paragraph</p></div><p>Outer text</p></body>", options);
    }
}

```
