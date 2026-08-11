---
id: fixture_php_real_world_product_page
language: php
target: php
level: typecheck
requires: []
side_effect: safe
---

```php title="PHP"
<?php

use HtmlToMarkdown\HtmlToMarkdown;
$result = HtmlToMarkdown::convert("<div class=\"product\"><h1>Wireless Keyboard Pro</h1><img src=\"https://example.com/keyboard.jpg\" alt=\"Wireless Keyboard Pro\"><p>The ultimate wireless keyboard for professionals. Features a comfortable layout with <strong>backlit keys</strong> and <em>ultra-long battery life</em>.</p><h2>Specifications</h2><table><thead><tr><th>Feature</th><th>Details</th></tr></thead><tbody><tr><td>Battery Life</td><td>Up to 12 months</td></tr><tr><td>Connectivity</td><td>Bluetooth 5.0</td></tr><tr><td>Key Travel</td><td>2mm</td></tr><tr><td>Weight</td><td>750g</td></tr></tbody></table><h2>What's in the Box</h2><ul><li>Wireless Keyboard Pro</li><li>USB-C charging cable</li><li>USB receiver dongle</li><li>Quick start guide</li></ul><h2>Compatibility</h2><p>Compatible with <strong>Windows</strong>, <strong>macOS</strong>, <strong>Linux</strong>, <strong>iOS</strong>, and <strong>Android</strong>.</p></div>");

```
