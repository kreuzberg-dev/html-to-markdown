```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
use HtmlToMarkdown\VisitorHandle;
$visitor = new class {
    public function visit_form(...$args) {
        return ['Custom' => "[FORM PLACEHOLDER]"];
    }
};
$options = \HtmlToMarkdown\ConversionOptions::from_json('{}');
$visitorHandle = \HtmlToMarkdown\VisitorHandle::from_php_object($visitor);
$options = $options->withVisitor($visitorHandle);
$result = HtmlToMarkdown::convert("<div><form action=\"/submit\" method=\"POST\"><label>Name: <input type=\"text\" name=\"name\"></label><button type=\"submit\">Submit</button></form></div>", $options);

```
