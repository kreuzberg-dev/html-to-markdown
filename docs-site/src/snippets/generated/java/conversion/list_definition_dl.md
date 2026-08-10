```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<dl><dt>Term One</dt><dd>Definition of term one.</dd><dt>Term Two</dt><dd>Definition of term two.</dd></dl>", ConversionOptions.builder().build());
    }
}

```
