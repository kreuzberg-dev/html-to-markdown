```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
use HtmlToMarkdown\ConversionOptions;
use HtmlToMarkdown\VisitorHandle;
$visitor = new class {
    public function visit_input(...$args) {
        return ['Custom' => "[INPUT:{input_type}]"];
    }
};
$options = \HtmlToMarkdown\ConversionOptions::from_json('{}');
$visitorHandle = \HtmlToMarkdown\VisitorHandle::from_php_object($visitor);
$options = $options->withVisitor($visitorHandle);
$result = HtmlToMarkdown::convert("<form><label>Username: <input type=\"text\" name=\"username\" value=\"\"></label><label>Password: <input type=\"password\" name=\"password\"></label></form>", $options);

```
