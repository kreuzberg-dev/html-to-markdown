```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{}');
  final result = await H2mBridge.convert('<table><tr><td>Product</td><td>Price</td></tr><tr><td>Apple</td><td>1.00</td></tr></table>', options: _options);
}

```
