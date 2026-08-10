```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>This is <mark>highlighted text</mark> in a sentence.</p>", ConversionOptions.builder().build());
    }
}

```
