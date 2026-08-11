---
id: fixture_php_options_preprocessing_remove_navigation_false_keeps_nav
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
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["preprocessing" => ["removeNavigation" => false]]));
$result = HtmlToMarkdown::convert("<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>", $options);

```
