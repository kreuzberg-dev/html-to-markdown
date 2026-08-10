```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
$options = \HtmlToMarkdown\ConversionOptions::from_json(json_encode(["skipImages" => true]));
$result = HtmlToMarkdown::convert("<p>Before <img src='test.jpg' alt='photo'> After</p>", $options);

```
