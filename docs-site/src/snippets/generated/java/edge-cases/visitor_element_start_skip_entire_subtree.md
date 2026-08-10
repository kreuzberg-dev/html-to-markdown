```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<div><h1>Title</h1><p>Content</p></div>", ConversionOptions.builder().build());
    }
}

```
