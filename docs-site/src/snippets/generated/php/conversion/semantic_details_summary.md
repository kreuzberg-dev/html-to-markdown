---
id: fixture_php_semantic_details_summary
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<details><summary>Click to expand</summary><p>Hidden content here.</p></details>");

```
