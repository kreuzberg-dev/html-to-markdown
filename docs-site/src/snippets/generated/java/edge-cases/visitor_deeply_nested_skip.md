```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<div><p>Outer <em>emphasis <strong>with bold <mark>and highlight</mark></strong></em> text</p></div>", ConversionOptions.builder().build());
    }
}

```
