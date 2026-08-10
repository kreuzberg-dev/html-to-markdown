```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<section><h3>Section Heading</h3><p>Section content.</p></section>", ConversionOptions.builder().build());
    }
}

```
