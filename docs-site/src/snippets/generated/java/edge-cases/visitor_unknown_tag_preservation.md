```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<article><p>Article text</p><x-custom>Custom element with content</x-custom><p>More article text</p></article>", ConversionOptions.builder().build());
    }
}

```
