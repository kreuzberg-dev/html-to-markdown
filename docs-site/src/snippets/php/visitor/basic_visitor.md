```php
use HtmlToMarkdown\HtmlToMarkdownApi;
use HtmlToMarkdown\ConversionOptions;
use HtmlToMarkdown\VisitorHandle;

// Visitors are duck-typed: define any subset of visit_* methods.
// Each method returns either 'Skip', ['Custom' => '...'], or null/'Continue'.
$visitor = new class {
    public function visit_link($ctx, $href, $text, $title) {
        return ['Custom' => "[{$text}]({$href})"];
    }

    public function visit_image($ctx, $src, $alt, $title) {
        return 'Skip';
    }
};

$visitorHandle = VisitorHandle::from_php_object($visitor);
$options = ConversionOptions::from_json('{}')->withVisitor($visitorHandle);

$result = HtmlToMarkdownApi::convert(
    '<a href="/page">Link</a><img src="pic.png" alt="pic">',
    $options
);
echo $result->content;
```
