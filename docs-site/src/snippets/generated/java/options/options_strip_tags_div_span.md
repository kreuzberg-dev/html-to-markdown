```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var optionsJson = "{\"strip_tags\":[\"div\",\"span\"]}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<div class='wrapper'><p>Inside div</p></div><p>Outside <span class='hl'>span text</span></p>", options);
    }
}

```
