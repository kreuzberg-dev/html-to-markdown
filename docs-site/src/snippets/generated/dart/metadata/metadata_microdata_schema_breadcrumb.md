```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"extract_metadata":true,"preprocessing":{"remove_navigation":false}}');
  final result = await H2mBridge.convert('<html><head><title>Navigation</title></head><body><nav itemscope itemtype="https://schema.org/BreadcrumbList"><span itemprop="itemListElement" itemscope itemtype="https://schema.org/ListItem"><a itemprop="item" href="https://example.com"><span itemprop="name">Home</span></a></span><span itemprop="itemListElement" itemscope itemtype="https://schema.org/ListItem"><a itemprop="item" href="https://example.com/products"><span itemprop="name">Products</span></a></span><span itemprop="itemListElement" itemscope itemtype="https://schema.org/ListItem"><span itemprop="name">Current Page</span></span></nav></body></html>', options: _options);
}

```
