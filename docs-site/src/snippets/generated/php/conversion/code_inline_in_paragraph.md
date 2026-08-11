---
id: fixture_php_code_inline_in_paragraph
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p>Call the <code>initialize()</code> method first.</p>");

```
