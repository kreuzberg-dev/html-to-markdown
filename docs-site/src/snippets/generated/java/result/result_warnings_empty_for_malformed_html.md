```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Unclosed paragraph<div>Mixed nesting</p></div>", ConversionOptions.builder().build());
    }
}

```
