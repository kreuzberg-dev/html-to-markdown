```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>", ConversionOptions.builder().build());
    }
}

```
