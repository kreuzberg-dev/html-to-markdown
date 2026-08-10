```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<a href=\"https://example.com\"><img src=\"icon.png\" alt=\"Icon\"></a>", ConversionOptions.builder().build());
    }
}

```
