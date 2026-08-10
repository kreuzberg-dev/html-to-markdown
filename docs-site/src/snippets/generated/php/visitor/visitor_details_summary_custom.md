```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
use HtmlToMarkdown\VisitorHandle;
$visitor = new class {
    public function visit_summary(...$args) {
        return ['Custom' => "[EXPANDABLE] {text}"];
    }
};
$options = \HtmlToMarkdown\ConversionOptions::from_json('{}');
$visitorHandle = \HtmlToMarkdown\VisitorHandle::from_php_object($visitor);
$options = $options->withVisitor($visitorHandle);
$result = HtmlToMarkdown::convert("<details><summary>Click to expand</summary><p>This content is initially hidden.</p><p>But can be revealed by the user.</p></details>", $options);

```
