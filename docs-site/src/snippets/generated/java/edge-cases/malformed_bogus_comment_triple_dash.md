```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<h1>One</h1>\n<!-- /// --->\n<p>Two</p>", ConversionOptions.builder().build());
    }
}

```
