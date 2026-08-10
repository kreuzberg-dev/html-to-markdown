```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<blockquote><p>Quote intro:</p><ul><li>Point one</li><li>Point two</li></ul></blockquote>", ConversionOptions.builder().build());
    }
}

```
