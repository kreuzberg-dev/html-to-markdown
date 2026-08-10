```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<blockquote><p>First paragraph.</p><p>Second paragraph.</p></blockquote>", ConversionOptions.builder().build());
    }
}

```
