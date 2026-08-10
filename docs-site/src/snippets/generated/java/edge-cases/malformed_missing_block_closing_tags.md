```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<div><h1>Title<p>First paragraph<p>Second paragraph</div>", ConversionOptions.builder().build());
    }
}

```
