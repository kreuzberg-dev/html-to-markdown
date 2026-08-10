```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<pre><code>plain code here</code></pre>", ConversionOptions.builder().build());
    }
}

```
