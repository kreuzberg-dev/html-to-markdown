```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<section><h2>Gallery</h2><figure><img src=\"photo1.jpg\" alt=\"Photo\"><figcaption>Beautiful sunset</figcaption></figure></section>", ConversionOptions.builder().build());
    }
}

```
