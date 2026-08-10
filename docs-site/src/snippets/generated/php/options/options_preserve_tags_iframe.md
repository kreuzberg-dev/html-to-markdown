```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["preserveTags" => ["iframe"]]));
$result = HtmlToMarkdown::convert("<p>Before</p><iframe src='video.html' width='560'></iframe><p>After</p>", $options);

```
