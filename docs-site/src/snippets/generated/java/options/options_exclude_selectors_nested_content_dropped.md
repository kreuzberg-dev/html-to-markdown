```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var optionsJson = "{\"exclude_selectors\":[\".sidebar\"]}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<body><aside class=\"sidebar\"><h2>Related</h2><p>Sidebar text</p></aside><main><p>Main text</p></main></body>", options);
    }
}

```
