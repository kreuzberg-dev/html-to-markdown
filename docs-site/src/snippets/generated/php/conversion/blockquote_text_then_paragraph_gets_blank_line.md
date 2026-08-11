---
id: fixture_php_blockquote_text_then_paragraph_gets_blank_line
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<blockquote>Just text, then <p>a paragraph</p></blockquote>");

```
