```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<html><head><title>Article</title><script type=\"application/ld+json\">{\"@context\":\"https://schema.org\",\"@type\":\"Article\",\"headline\":\"My Article\",\"author\":{\"@type\":\"Person\",\"name\":\"Jane Doe\"},\"datePublished\":\"2024-01-15\"}</script></head><body><h1>My Article</h1><p>Article body text.</p></body></html>", ConversionOptions.builder().build());
    }
}

```
