---
id: fixture_php_style_tags_only
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<html><head><style>body { color: red; }</style></head><body><style>.foo { margin: 0; }</style></body></html>");

```
