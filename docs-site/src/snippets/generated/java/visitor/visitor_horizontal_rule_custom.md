```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<h1>Section A</h1><p>Content A</p><hr><h1>Section B</h1><p>Content B</p>", ConversionOptions.builder().build());
    }
}

```
