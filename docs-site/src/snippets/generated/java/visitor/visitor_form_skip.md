```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Before form</p><form><input type=\"email\" name=\"email\"></form><p>After form</p>", ConversionOptions.builder().build());
    }
}

```
