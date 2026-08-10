```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"autolinks":false}');
  final result = await H2mBridge.convert('<p><a href=\'https://example.com\'>https://example.com</a></p>', options: _options);
}

```
