```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Em dash&mdash;used for parenthetical remarks&mdash;is common. Ellipsis&hellip; indicates omission. Non-breaking&nbsp;space.</p>", ConversionOptions.builder().build());
    }
}

```
