```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<a href=\"https://old-cdn.com/file.pdf\">Download</a>", ConversionOptions.builder().build());
    }
}

```
