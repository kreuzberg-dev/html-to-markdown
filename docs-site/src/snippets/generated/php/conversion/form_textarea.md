---
id: fixture_php_form_textarea
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["preprocessing" => ["removeForms" => false]]));
$result = HtmlToMarkdown::convert("<form><label>Message:</label><textarea>Default text content</textarea></form>", $options);

```
