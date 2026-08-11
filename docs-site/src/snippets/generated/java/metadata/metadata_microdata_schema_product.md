---
id: fixture_java_metadata_microdata_schema_product
language: java
target: java
level: typecheck
requires: []
side_effect: safe
---

```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var optionsJson = "{\"extract_metadata\":true}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<html><head><title>Product</title></head><body><div itemscope itemtype=\"https://schema.org/Product\"><h1 itemprop=\"name\">Awesome Widget</h1><span itemprop=\"description\">The best widget on the market</span><span itemprop=\"price\">29.99</span><span itemprop=\"priceCurrency\">USD</span><img itemprop=\"image\" src=\"widget.jpg\" alt=\"Widget\"><span itemprop=\"ratingValue\">4.5</span></div></body></html>", options);
    }
}

```
