---
id: fixture_php_encoding_html_entities
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<p>&amp; &lt; &gt; &nbsp; &quot; &apos;</p>");

```
