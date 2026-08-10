```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<blockquote><p>Outer quote.</p><blockquote><p>Inner quote.</p></blockquote></blockquote>", ConversionOptions.builder().build());
    }
}

```
