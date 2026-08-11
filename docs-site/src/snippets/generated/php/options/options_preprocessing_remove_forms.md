---
id: fixture_php_options_preprocessing_remove_forms
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["preprocessing" => ["removeForms" => true]]));
$result = HtmlToMarkdown::convert("<p>Before</p><form><input type='text'/><button>Submit</button></form><p>After</p>", $options);

```
