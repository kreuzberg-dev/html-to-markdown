```php
use HtmlToMarkdown\HtmlToMarkdownApi;
use HtmlToMarkdown\ConversionOptions;

$options = ConversionOptions::from_json(json_encode([
    'headingStyle' => 'Atx',
    'listIndentWidth' => 2,
]));

$result = HtmlToMarkdownApi::convert('<h1>Hello</h1>', $options);
echo $result->content;
```
