```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var optionsJson = "{\"exclude_selectors\":[\"#ad-container\"]}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<body><div id=\"ad-container\">Buy stuff</div><p>Article text</p></body>", options);
    }
}

```
