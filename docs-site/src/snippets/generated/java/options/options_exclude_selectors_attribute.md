```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var optionsJson = "{\"exclude_selectors\":[\"[role='complementary']\"]}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<body><div role=\"complementary\">Sidebar</div><p>Primary text</p></body>", options);
    }
}

```
