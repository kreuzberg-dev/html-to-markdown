```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<div><custom-tag>Custom content</custom-tag></div>", ConversionOptions.builder().build());
    }
}

```
