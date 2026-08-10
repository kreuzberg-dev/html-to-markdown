```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<ul><li><input type=\"checkbox\" checked> Done task</li><li><input type=\"checkbox\"> Pending task</li></ul>", ConversionOptions.builder().build());
    }
}

```
