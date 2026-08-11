```php
use HtmlToMarkdown\HtmlToMarkdownApi;

// Binary data (detected via magic bytes) is rejected before parsing.
$html = '%PDF-1.4 not actually HTML';

try {
    $result = HtmlToMarkdownApi::convert($html);
    echo $result->content;
} catch (\Exception $e) {
    // Native conversion failures surface as \Exception, prefixed with the
    // Rust error variant, e.g. "[InvalidInput] Invalid input: ...".
    fwrite(STDERR, 'conversion failed: ' . $e->getMessage() . "\n");
}
```
