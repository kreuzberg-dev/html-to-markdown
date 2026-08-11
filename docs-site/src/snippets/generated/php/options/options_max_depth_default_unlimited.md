---
id: fixture_php_options_max_depth_default_unlimited
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<div><div><div><div><p>Deep content</p></div></div></div></div>");

```
