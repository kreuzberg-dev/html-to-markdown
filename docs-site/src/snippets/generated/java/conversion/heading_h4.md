```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<h4>Heading 4</h4>", ConversionOptions.builder().build());
    }
}

```
