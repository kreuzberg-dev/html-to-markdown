```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Before <a href=\"https://example.com\">link text</a> after</p>", ConversionOptions.builder().build());
    }
}

```
