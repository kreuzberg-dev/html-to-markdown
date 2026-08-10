```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Before rule.</p><hr><p>After rule.</p>", ConversionOptions.builder().build());
    }
}

```
