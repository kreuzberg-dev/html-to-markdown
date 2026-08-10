```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var optionsJson = "{\"extract_metadata\":true}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<html><head><title>Scholarly Work</title><meta name=\"DC.title\" content=\"Principles of Knowledge Management\"><meta name=\"DC.creator\" content=\"Dr. Alice Johnson\"><meta name=\"DC.date\" content=\"2023-06-15\"><meta name=\"DC.subject\" content=\"Knowledge Management\"><meta name=\"DC.publisher\" content=\"Academic Press\"></head><body><p>This is a scholarly article.</p></body></html>", options);
    }
}

```
