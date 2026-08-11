---
id: fixture_php_encoding_unicode_emoji
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p>Hello 🌍 World 🚀</p><p>Stars: ⭐ ✨</p>");

```
