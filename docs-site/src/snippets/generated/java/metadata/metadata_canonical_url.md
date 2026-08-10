```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var optionsJson = "{\"extract_metadata\":true}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<html><head><title>Page</title><link rel=\"canonical\" href=\"https://example.com/canonical-page\"></head><body><p>Content</p></body></html>", options);
    }
}

```
