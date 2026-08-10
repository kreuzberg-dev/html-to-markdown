```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var optionsJson = "{\"include_document_structure\":true}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<h1>Chapter One</h1><h2>Section A</h2><p>Section A content.</p><h1>Chapter Two</h1><h2>Section B</h2><p>Section B content.</p>", options);
    }
}

```
