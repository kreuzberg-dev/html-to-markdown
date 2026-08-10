```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Address Line 1<br>Address Line 2<br>Address Line 3</p>", ConversionOptions.builder().build());
    }
}

```
