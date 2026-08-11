---
id: fixture_php_issue_396_backticks_blank_line_after_fence
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["codeBlockStyle" => "Backticks"]));
$result = HtmlToMarkdown::convert("<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>", $options);

```
