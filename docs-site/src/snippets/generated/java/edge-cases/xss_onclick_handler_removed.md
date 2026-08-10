```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p><a href=\"https://example.com\" onclick=\"alert('xss')\">Click me</a></p><button onmouseover=\"steal_data()\">Hover me</button>", ConversionOptions.builder().build());
    }
}

```
