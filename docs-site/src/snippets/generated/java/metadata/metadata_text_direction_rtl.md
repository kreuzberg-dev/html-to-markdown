```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var optionsJson = "{\"extract_metadata\":true}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<html lang=\"ar\" dir=\"rtl\"><head><title>RTL Document</title></head><body><p>This is right-to-left text.</p></body></html>", options);
    }
}

```
