---
id: fixture_php_semantic_section_with_heading
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<section><h3>Section Heading</h3><p>Section content.</p></section>");

```
