```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Normal text with <u>underlined part</u> and more text.</p>", ConversionOptions.builder().build());
    }
}

```
