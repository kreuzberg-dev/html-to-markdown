```java
import io.xberg.htmltomarkdown.HtmlToMarkdown;
import io.xberg.htmltomarkdown.ConversionOptions;
import io.xberg.htmltomarkdown.ConversionResult;
import io.xberg.htmltomarkdown.HtmlToMarkdownRsException;

public class MetadataExample {
    public static void main(String[] args) throws HtmlToMarkdownRsException {
        String html = """
            <html><head><title>My Page</title></head>
            <body><h1>Hello</h1><a href="https://example.com">Link</a></body></html>
            """;

        ConversionOptions options = ConversionOptions.builder()
            .withExtractMetadata(true)
            .build();
        ConversionResult result = HtmlToMarkdown.convert(html, options);
        System.out.println("Markdown: " + result.content());
        System.out.println("Title: " + result.metadata().document().title());
        System.out.println("Links: " + result.metadata().links());
    }
}
```
