```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>This has <strong>bold</strong>, <em>italic</em>, and a <a href=\"https://example.com\">link</a>.</p>", ConversionOptions.builder().build());
    }
}

```
