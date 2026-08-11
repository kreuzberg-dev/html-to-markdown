---
id: fixture_php_malformed_missing_block_closing_tags
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<div><h1>Title<p>First paragraph<p>Second paragraph</div>");

```
