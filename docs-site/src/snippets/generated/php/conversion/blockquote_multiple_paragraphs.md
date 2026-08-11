---
id: fixture_php_blockquote_multiple_paragraphs
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<blockquote><p>First paragraph.</p><p>Second paragraph.</p></blockquote>");

```
