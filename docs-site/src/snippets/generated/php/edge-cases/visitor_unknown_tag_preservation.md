---
id: fixture_php_visitor_unknown_tag_preservation
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
use HtmlToMarkdown\VisitorHandle;
$visitor = new class {
    public function visit_custom_element(...$args) {
        return 'PreserveHtml';
    }
};
$options = \HtmlToMarkdown\ConversionOptions::from_json('{}');
$visitorHandle = \HtmlToMarkdown\VisitorHandle::from_php_object($visitor);
$options = $options->withVisitor($visitorHandle);
$result = HtmlToMarkdown::convert("<article><p>Article text</p><x-custom>Custom element with content</x-custom><p>More article text</p></article>", $options);

```
