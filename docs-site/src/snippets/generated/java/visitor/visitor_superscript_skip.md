```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>The equation x<sup>3</sup> + y<sup>3</sup> = z<sup>3</sup> has no solutions.</p>", ConversionOptions.builder().build());
    }
}

```
