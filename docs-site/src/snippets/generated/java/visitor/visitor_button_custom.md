```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Confirm action: <button type=\"submit\">Click me</button> or <button type=\"reset\">Cancel</button></p>", ConversionOptions.builder().build());
    }
}

```
