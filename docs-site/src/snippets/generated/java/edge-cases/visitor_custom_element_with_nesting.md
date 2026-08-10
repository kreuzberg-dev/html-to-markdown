```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<div><custom-widget data-value=\"123\"><p>Widget content here</p><span>With nested elements</span></custom-widget></div>", ConversionOptions.builder().build());
    }
}

```
