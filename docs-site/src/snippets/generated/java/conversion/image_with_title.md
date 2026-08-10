```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<img src=\"chart.png\" alt=\"Sales chart\" title=\"Q3 Sales\">", ConversionOptions.builder().build());
    }
}

```
