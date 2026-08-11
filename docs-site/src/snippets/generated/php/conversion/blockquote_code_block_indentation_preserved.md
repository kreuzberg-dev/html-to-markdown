---
id: fixture_php_blockquote_code_block_indentation_preserved
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<blockquote><pre><code>line1\n    line2 indented</code></pre></blockquote>");

```
