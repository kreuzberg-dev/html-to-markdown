```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<table><tr><td>Product</td><td>Price</td></tr><tr><td>Apple</td><td>1.00</td></tr></table>", ConversionOptions.builder().build());
    }
}

```
