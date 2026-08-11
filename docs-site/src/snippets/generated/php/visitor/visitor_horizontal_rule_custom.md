---
id: fixture_php_visitor_horizontal_rule_custom
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
    public function visit_horizontal_rule(...$args) {
        return ['Custom' => "\n[DIVIDER]\n"];
    }
};
$options = \HtmlToMarkdown\ConversionOptions::from_json('{}');
$visitorHandle = \HtmlToMarkdown\VisitorHandle::from_php_object($visitor);
$options = $options->withVisitor($visitorHandle);
$result = HtmlToMarkdown::convert("<h1>Section A</h1><p>Content A</p><hr><h1>Section B</h1><p>Content B</p>", $options);

```
