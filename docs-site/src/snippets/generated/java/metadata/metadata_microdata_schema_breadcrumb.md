```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var optionsJson = "{\"extract_metadata\":true,\"preprocessing\":{\"remove_navigation\":false}}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<html><head><title>Navigation</title></head><body><nav itemscope itemtype=\"https://schema.org/BreadcrumbList\"><span itemprop=\"itemListElement\" itemscope itemtype=\"https://schema.org/ListItem\"><a itemprop=\"item\" href=\"https://example.com\"><span itemprop=\"name\">Home</span></a></span><span itemprop=\"itemListElement\" itemscope itemtype=\"https://schema.org/ListItem\"><a itemprop=\"item\" href=\"https://example.com/products\"><span itemprop=\"name\">Products</span></a></span><span itemprop=\"itemListElement\" itemscope itemtype=\"https://schema.org/ListItem\"><span itemprop=\"name\">Current Page</span></span></nav></body></html>", options);
    }
}

```
