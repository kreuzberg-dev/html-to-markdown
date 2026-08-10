```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
use HtmlToMarkdown\VisitorHandle;
$visitor = new class {
    public function visit_figcaption(...$args) {
        return ['Custom' => "*{text}*"];
    }
};
$options = \HtmlToMarkdown\ConversionOptions::from_json('{}');
$visitorHandle = \HtmlToMarkdown\VisitorHandle::from_php_object($visitor);
$options = $options->withVisitor($visitorHandle);
$result = HtmlToMarkdown::convert("<article><h1>Article Title</h1><p>Introduction paragraph.</p><figure><img src=\"diagram.png\" alt=\"System architecture diagram\"><figcaption>Figure 1: System Architecture</figcaption></figure><p>Explanation of the figure.</p></article>", $options);

```
